//! GGA_C_WI kernel -- incremental derivative structure.

//! unpol: preamble=26 lines
//!   exc: shared=0, delta=26, outputs=1
//!   vxc: shared=26, delta=30, outputs=3
//!   fxc: shared=56, delta=64, outputs=6
//!   kxc: shared=120, delta=109, outputs=10
//!   lxc: shared=229, delta=22, outputs=15
//! pol: preamble=28 lines
//!   exc: shared=0, delta=28, outputs=1
//!   vxc: shared=28, delta=36, outputs=6
//!   fxc: shared=64, delta=98, outputs=21
//!   kxc: shared=162, delta=223, outputs=56
//!   lxc: shared=385, delta=295, outputs=126

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
