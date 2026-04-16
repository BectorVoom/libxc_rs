//! GGA_C_ACGGA kernel — split into per-function files.

pub mod exc_unpol;
pub mod vxc_unpol;
pub mod fxc_unpol;
#[cfg(feature = "order-kxc")]
pub mod kxc_unpol;
#[cfg(feature = "order-lxc")]
pub mod lxc_unpol;
pub mod exc_pol;
pub mod vxc_pol;
pub mod fxc_pol;
#[cfg(feature = "order-kxc")]
pub mod kxc_pol;
#[cfg(feature = "order-lxc")]
pub mod lxc_pol;
