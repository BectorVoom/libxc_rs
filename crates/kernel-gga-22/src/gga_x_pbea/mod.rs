//! GGA_X_PBEA kernel -- incremental derivative structure.

//! unpol: preamble=22 lines
//!   exc: shared=0, delta=22, outputs=1
//!   vxc: shared=22, delta=9, outputs=3
//!   fxc: shared=31, delta=18, outputs=6
//!   kxc: shared=49, delta=18, outputs=10
//!   lxc: shared=67, delta=13, outputs=15
//! pol: preamble=43 lines
//!   exc: shared=0, delta=43, outputs=1
//!   vxc: shared=43, delta=45, outputs=6
//!   fxc: shared=88, delta=110, outputs=21
//!   kxc: shared=198, delta=209, outputs=56
//!   lxc: shared=407, delta=270, outputs=126

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
