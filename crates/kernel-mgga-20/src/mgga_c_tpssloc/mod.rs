//! MGGA_C_TPSSLOC kernel -- incremental derivative structure.

//! unpol: preamble=315 lines
//!   exc: shared=0, delta=315, outputs=1
//!   vxc: shared=315, delta=423, outputs=5
//!   fxc: shared=738, delta=891, outputs=15
//!   kxc: shared=1629, delta=1673, outputs=35
//!   lxc: shared=3302, delta=1150, outputs=70
//! pol: preamble=389 lines
//!   exc: shared=0, delta=389, outputs=1
//!   vxc: shared=389, delta=909, outputs=10
//!   fxc: shared=1298, delta=3217, outputs=55
//!   kxc: shared=4515, delta=11316, outputs=220
//!   lxc: shared=15831, delta=26393, outputs=715

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
