//! MGGA_X_PKZB kernel -- incremental derivative structure.

//! unpol: preamble=44 lines
//!   exc: shared=0, delta=44, outputs=1
//!   vxc: shared=44, delta=29, outputs=5
//!   fxc: shared=73, delta=51, outputs=15
//!   kxc: shared=124, delta=95, outputs=35
//!   lxc: shared=219, delta=96, outputs=70
//! pol: preamble=72 lines
//!   exc: shared=0, delta=72, outputs=1
//!   vxc: shared=72, delta=79, outputs=10
//!   fxc: shared=151, delta=200, outputs=55
//!   kxc: shared=351, delta=503, outputs=220
//!   lxc: shared=854, delta=946, outputs=715

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
