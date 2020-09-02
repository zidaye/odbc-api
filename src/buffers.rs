//! This module contains buffers intended to be bound to ODBC statement handles.
mod text_column;
mod text_row_set;
mod fixed_sized_c_type;

use odbc_sys::{CDataType, Len, Pointer};

pub use self::{text_column::TextColumn, text_row_set::TextRowSet, fixed_sized_c_type::FixedSizedCType};

/// Arguments used to descripbe the buffer then binding this column to a cursor.
pub struct BindColParameters {
    /// The identifier of the C data type of the `value` buffer. When it is retrieving data from the
    /// data source with `fetch`, the driver converts the data to this type. When it sends data to
    /// the source, the driver converts the data from this type.
    pub target_type: CDataType,
    /// Pointer to the data buffer to bind to the column.Start address of the value buffer.
    pub target_value: Pointer,
    /// Length of a target value in bytes. For a single element in case of block fetching data.
    pub target_length: Len,
    /// Buffer is going to hold length or indicator values. Can be null if the column is not
    /// nullable and elemnts are of fixed length.
    pub indicator: *mut Len,
}

/// A type implementing this trait can be bound to a column of a cursor.
pub unsafe trait ColumnBuffer {
    /// Arguments used to descripbe the buffer then binding this column to a cursor.
    fn bind_arguments(&mut self) -> BindColParameters;
}
