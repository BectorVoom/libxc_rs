//! MGGA_X_TASK kernel -- incremental derivative structure.

//! unpol: preamble=108 lines
//!   exc: shared=0, delta=108, outputs=1
//!   vxc: shared=108, delta=87, outputs=5
//!   fxc: shared=195, delta=222, outputs=15
//!   kxc: shared=417, delta=524, outputs=35
//!   lxc: shared=941, delta=372, outputs=70
//! pol: preamble=178 lines
//!   exc: shared=0, delta=178, outputs=1
//!   vxc: shared=178, delta=187, outputs=10
//!   fxc: shared=365, delta=522, outputs=55
//!   kxc: shared=887, delta=1332, outputs=220
//!   lxc: shared=2219, delta=1480, outputs=715

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
