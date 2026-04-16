//! GGA_X_EV93 kernel -- incremental derivative structure.

//! unpol: preamble=55 lines
//!   exc: shared=0, delta=55, outputs=1
//!   vxc: shared=55, delta=30, outputs=3
//!   fxc: shared=85, delta=44, outputs=6
//!   kxc: shared=129, delta=73, outputs=10
//!   lxc: shared=202, delta=37, outputs=15
//! pol: preamble=88 lines
//!   exc: shared=0, delta=88, outputs=1
//!   vxc: shared=88, delta=89, outputs=6
//!   fxc: shared=177, delta=182, outputs=21
//!   kxc: shared=359, delta=407, outputs=56
//!   lxc: shared=766, delta=466, outputs=126

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
