//! MGGA_X_TAU_HCTH kernel -- incremental derivative structure.

//! unpol: preamble=74 lines
//!   exc: shared=0, delta=74, outputs=1
//!   vxc: shared=74, delta=60, outputs=5
//!   fxc: shared=134, delta=77, outputs=15
//!   kxc: shared=211, delta=104, outputs=35
//!   lxc: shared=315, delta=105, outputs=70
//! pol: preamble=131 lines
//!   exc: shared=0, delta=131, outputs=1
//!   vxc: shared=131, delta=131, outputs=10
//!   fxc: shared=262, delta=236, outputs=55
//!   kxc: shared=498, delta=495, outputs=220
//!   lxc: shared=993, delta=856, outputs=715

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
