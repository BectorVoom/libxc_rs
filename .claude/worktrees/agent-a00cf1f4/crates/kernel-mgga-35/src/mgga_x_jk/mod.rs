//! MGGA_X_JK kernel -- incremental derivative structure.

//! unpol: preamble=51 lines
//!   exc: shared=0, delta=51, outputs=1
//!   vxc: shared=51, delta=54, outputs=5
//!   fxc: shared=105, delta=108, outputs=15
//!   kxc: shared=213, delta=229, outputs=35
//!   lxc: shared=442, delta=146, outputs=70
//! pol: preamble=84 lines
//!   exc: shared=0, delta=84, outputs=1
//!   vxc: shared=84, delta=107, outputs=10
//!   fxc: shared=191, delta=298, outputs=55
//!   kxc: shared=489, delta=813, outputs=220
//!   lxc: shared=1302, delta=1053, outputs=715

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
