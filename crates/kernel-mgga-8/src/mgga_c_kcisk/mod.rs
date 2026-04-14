//! MGGA_C_KCISK kernel -- incremental derivative structure.

//! unpol: preamble=250 lines
//!   exc: shared=0, delta=250, outputs=1
//!   vxc: shared=250, delta=328, outputs=5
//!   fxc: shared=578, delta=612, outputs=15
//!   kxc: shared=1190, delta=1125, outputs=35
//!   lxc: shared=2315, delta=1188, outputs=70
//! pol: preamble=471 lines
//!   exc: shared=0, delta=471, outputs=1
//!   vxc: shared=471, delta=1083, outputs=10
//!   fxc: shared=1554, delta=3486, outputs=55
//!   kxc: shared=5040, delta=11135, outputs=220
//!   lxc: shared=16175, delta=20902, outputs=715

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
