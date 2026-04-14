//! MGGA_X_RLDA kernel -- incremental derivative structure.

//! unpol: preamble=26 lines
//!   exc: shared=0, delta=26, outputs=1
//!   vxc: shared=26, delta=20, outputs=5
//!   fxc: shared=46, delta=32, outputs=15
//!   kxc: shared=78, delta=55, outputs=35
//!   lxc: shared=133, delta=71, outputs=70
//! pol: preamble=44 lines
//!   exc: shared=0, delta=44, outputs=1
//!   vxc: shared=44, delta=57, outputs=10
//!   fxc: shared=101, delta=153, outputs=55
//!   kxc: shared=254, delta=396, outputs=220
//!   lxc: shared=650, delta=890, outputs=715

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
