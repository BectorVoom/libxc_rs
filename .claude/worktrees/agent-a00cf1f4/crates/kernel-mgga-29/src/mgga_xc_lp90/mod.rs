//! MGGA_XC_LP90 kernel -- incremental derivative structure.

//! unpol: preamble=10 lines
//!   exc: shared=0, delta=10, outputs=1
//!   vxc: shared=10, delta=10, outputs=5
//!   fxc: shared=20, delta=19, outputs=15
//!   kxc: shared=39, delta=34, outputs=35
//!   lxc: shared=73, delta=35, outputs=70
//! pol: preamble=30 lines
//!   exc: shared=0, delta=30, outputs=1
//!   vxc: shared=30, delta=39, outputs=10
//!   fxc: shared=69, delta=112, outputs=55
//!   kxc: shared=181, delta=312, outputs=220
//!   lxc: shared=493, delta=654, outputs=715

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
