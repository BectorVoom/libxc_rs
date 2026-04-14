//! MGGA_X_M08 kernel -- incremental derivative structure.

//! unpol: preamble=114 lines
//!   exc: shared=0, delta=114, outputs=1
//!   vxc: shared=114, delta=125, outputs=5
//!   fxc: shared=239, delta=211, outputs=15
//!   kxc: shared=450, delta=264, outputs=35
//!   lxc: shared=714, delta=335, outputs=70
//! pol: preamble=195 lines
//!   exc: shared=0, delta=195, outputs=1
//!   vxc: shared=195, delta=205, outputs=10
//!   fxc: shared=400, delta=454, outputs=55
//!   kxc: shared=854, delta=857, outputs=220
//!   lxc: shared=1711, delta=1240, outputs=715

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
