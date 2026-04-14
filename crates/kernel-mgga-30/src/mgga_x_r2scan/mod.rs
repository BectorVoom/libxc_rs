//! MGGA_X_R2SCAN kernel -- incremental derivative structure.

//! unpol: preamble=84 lines
//!   exc: shared=0, delta=84, outputs=1
//!   vxc: shared=84, delta=104, outputs=5
//!   fxc: shared=188, delta=278, outputs=15
//!   kxc: shared=466, delta=522, outputs=35
//!   lxc: shared=988, delta=460, outputs=70
//! pol: preamble=154 lines
//!   exc: shared=0, delta=154, outputs=1
//!   vxc: shared=154, delta=227, outputs=10
//!   fxc: shared=381, delta=671, outputs=55
//!   kxc: shared=1052, delta=1480, outputs=220
//!   lxc: shared=2532, delta=2073, outputs=715

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
