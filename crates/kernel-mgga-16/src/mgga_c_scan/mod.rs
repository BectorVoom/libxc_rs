//! MGGA_C_SCAN kernel -- incremental derivative structure.

//! unpol: preamble=108 lines
//!   exc: shared=0, delta=108, outputs=1
//!   vxc: shared=108, delta=130, outputs=5
//!   fxc: shared=238, delta=329, outputs=15
//!   kxc: shared=567, delta=655, outputs=35
//!   lxc: shared=1222, delta=634, outputs=70
//! pol: preamble=167 lines
//!   exc: shared=0, delta=167, outputs=1
//!   vxc: shared=167, delta=285, outputs=10
//!   fxc: shared=452, delta=971, outputs=55
//!   kxc: shared=1423, delta=3022, outputs=220
//!   lxc: shared=4445, delta=7327, outputs=715

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
