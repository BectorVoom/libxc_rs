//! GGA_X_LG93 kernel -- incremental derivative structure.

//! unpol: preamble=57 lines
//!   exc: shared=0, delta=57, outputs=1
//!   vxc: shared=57, delta=37, outputs=3
//!   fxc: shared=94, delta=60, outputs=6
//!   kxc: shared=154, delta=101, outputs=10
//!   lxc: shared=255, delta=52, outputs=15
//! pol: preamble=97 lines
//!   exc: shared=0, delta=97, outputs=1
//!   vxc: shared=97, delta=99, outputs=6
//!   fxc: shared=196, delta=210, outputs=21
//!   kxc: shared=406, delta=470, outputs=56
//!   lxc: shared=876, delta=591, outputs=126

pub mod exc_unpol;
pub mod vxc_unpol;
pub mod fxc_unpol;
pub mod kxc_unpol;
pub mod lxc_unpol;
pub mod exc_pol;
pub mod vxc_pol;
pub mod fxc_pol;
pub mod kxc_pol;
pub mod lxc_pol;
