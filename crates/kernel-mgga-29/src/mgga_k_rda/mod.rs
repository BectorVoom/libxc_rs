//! MGGA_K_RDA kernel -- incremental derivative structure.

//! unpol: preamble=70 lines
//!   exc: shared=0, delta=70, outputs=1
//!   vxc: shared=70, delta=67, outputs=5
//!   fxc: shared=137, delta=178, outputs=15
//!   kxc: shared=315, delta=356, outputs=35
//!   lxc: shared=671, delta=252, outputs=70
//! pol: preamble=116 lines
//!   exc: shared=0, delta=116, outputs=1
//!   vxc: shared=116, delta=132, outputs=10
//!   fxc: shared=248, delta=386, outputs=55
//!   kxc: shared=634, delta=865, outputs=220
//!   lxc: shared=1499, delta=1194, outputs=715

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
