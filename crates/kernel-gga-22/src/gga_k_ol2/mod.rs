//! GGA_K_OL2 kernel -- incremental derivative structure.

//! unpol: preamble=32 lines
//!   exc: shared=0, delta=32, outputs=1
//!   vxc: shared=32, delta=20, outputs=3
//!   fxc: shared=52, delta=31, outputs=6
//!   kxc: shared=83, delta=36, outputs=10
//!   lxc: shared=119, delta=15, outputs=15
//! pol: preamble=61 lines
//!   exc: shared=0, delta=61, outputs=1
//!   vxc: shared=61, delta=54, outputs=6
//!   fxc: shared=115, delta=118, outputs=21
//!   kxc: shared=233, delta=199, outputs=56
//!   lxc: shared=432, delta=235, outputs=126

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
