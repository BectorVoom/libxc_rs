//! MGGA_X_M06L kernel -- incremental derivative structure.

//! unpol: preamble=113 lines
//!   exc: shared=0, delta=113, outputs=1
//!   vxc: shared=113, delta=105, outputs=5
//!   fxc: shared=218, delta=179, outputs=15
//!   kxc: shared=397, delta=327, outputs=35
//!   lxc: shared=724, delta=308, outputs=70
//! pol: preamble=194 lines
//!   exc: shared=0, delta=194, outputs=1
//!   vxc: shared=194, delta=201, outputs=10
//!   fxc: shared=395, delta=447, outputs=55
//!   kxc: shared=842, delta=960, outputs=220
//!   lxc: shared=1802, delta=1246, outputs=715

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
