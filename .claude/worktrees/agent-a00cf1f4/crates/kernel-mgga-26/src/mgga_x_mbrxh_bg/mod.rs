//! MGGA_X_MBRXH_BG kernel -- incremental derivative structure.

//! unpol: preamble=50 lines
//!   exc: shared=0, delta=50, outputs=1
//!   vxc: shared=50, delta=72, outputs=5
//!   fxc: shared=122, delta=314, outputs=15
//!   kxc: shared=436, delta=1438, outputs=35
//!   lxc: shared=1874, delta=3062, outputs=70
//! pol: preamble=89 lines
//!   exc: shared=0, delta=89, outputs=1
//!   vxc: shared=89, delta=160, outputs=10
//!   fxc: shared=249, delta=740, outputs=55
//!   kxc: shared=989, delta=3513, outputs=220
//!   lxc: shared=4502, delta=9615, outputs=715

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
