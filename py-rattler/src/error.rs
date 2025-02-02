use pyo3::exceptions::PyException;
use pyo3::{create_exception, PyErr};
use rattler_conda_types::ParseVersionError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PyRattlerError {
    #[error(transparent)]
    InvalidVersion(#[from] ParseVersionError),
}

impl From<PyRattlerError> for PyErr {
    fn from(value: PyRattlerError) -> Self {
        match value {
            PyRattlerError::InvalidVersion(err) => {
                InvalidVersionException::new_err(err.to_string())
            }
        }
    }
}

create_exception!(exceptions, InvalidVersionException, PyException);
