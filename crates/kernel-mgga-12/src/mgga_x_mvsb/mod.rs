//! MGGA_X_MVSB kernel -- incremental derivative structure.

//! unpol: preamble=55 lines
//!   exc: shared=0, delta=55, outputs=1
//!   vxc: shared=55, delta=62, outputs=5
//!   fxc: shared=117, delta=157, outputs=15
//!   kxc: shared=274, delta=317, outputs=35
//!   lxc: shared=591, delta=232, outputs=70
//! pol: preamble=105 lines
//!   exc: shared=0, delta=105, outputs=1
//!   vxc: shared=105, delta=143, outputs=10
//!   fxc: shared=248, delta=413, outputs=55
//!   kxc: shared=661, delta=970, outputs=220
//!   lxc: shared=1631, delta=1432, outputs=715

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
