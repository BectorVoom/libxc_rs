//! HYB_MGGA_X_DLDF kernel -- incremental derivative structure.

//! unpol: preamble=45 lines
//!   exc: shared=0, delta=45, outputs=1
//!   vxc: shared=45, delta=25, outputs=5
//!   fxc: shared=70, delta=60, outputs=15
//!   kxc: shared=130, delta=90, outputs=35
//!   lxc: shared=220, delta=86, outputs=70
//! pol: preamble=83 lines
//!   exc: shared=0, delta=83, outputs=1
//!   vxc: shared=83, delta=79, outputs=10
//!   fxc: shared=162, delta=215, outputs=55
//!   kxc: shared=377, delta=515, outputs=220
//!   lxc: shared=892, delta=952, outputs=715

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
