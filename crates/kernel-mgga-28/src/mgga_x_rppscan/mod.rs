//! MGGA_X_RPPSCAN kernel -- incremental derivative structure.

//! unpol: preamble=83 lines
//!   exc: shared=0, delta=83, outputs=1
//!   vxc: shared=83, delta=102, outputs=5
//!   fxc: shared=185, delta=267, outputs=15
//!   kxc: shared=452, delta=560, outputs=35
//!   lxc: shared=1012, delta=548, outputs=70
//! pol: preamble=154 lines
//!   exc: shared=0, delta=154, outputs=1
//!   vxc: shared=154, delta=213, outputs=10
//!   fxc: shared=367, delta=660, outputs=55
//!   kxc: shared=1027, delta=1533, outputs=220
//!   lxc: shared=2560, delta=2206, outputs=715

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
