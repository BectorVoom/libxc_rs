//! MGGA_X_REVTM kernel -- incremental derivative structure.

//! unpol: preamble=76 lines
//!   exc: shared=0, delta=76, outputs=1
//!   vxc: shared=76, delta=97, outputs=5
//!   fxc: shared=173, delta=247, outputs=15
//!   kxc: shared=420, delta=686, outputs=35
//!   lxc: shared=1106, delta=786, outputs=70
//! pol: preamble=137 lines
//!   exc: shared=0, delta=137, outputs=1
//!   vxc: shared=137, delta=207, outputs=10
//!   fxc: shared=344, delta=568, outputs=55
//!   kxc: shared=912, delta=1575, outputs=220
//!   lxc: shared=2487, delta=2203, outputs=715

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
