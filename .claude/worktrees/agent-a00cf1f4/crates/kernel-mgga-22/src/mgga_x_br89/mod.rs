//! MGGA_X_BR89 kernel -- incremental derivative structure.

//! unpol: preamble=65 lines
//!   exc: shared=0, delta=65, outputs=1
//!   vxc: shared=65, delta=104, outputs=5
//!   fxc: shared=169, delta=466, outputs=15
//!   kxc: shared=635, delta=2597, outputs=35
//!   lxc: shared=3232, delta=8872, outputs=70
//! pol: preamble=120 lines
//!   exc: shared=0, delta=120, outputs=1
//!   vxc: shared=120, delta=228, outputs=10
//!   fxc: shared=348, delta=1068, outputs=55
//!   kxc: shared=1416, delta=6093, outputs=220
//!   lxc: shared=7509, delta=24543, outputs=715

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
