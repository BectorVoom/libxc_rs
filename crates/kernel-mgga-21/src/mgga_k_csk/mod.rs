//! MGGA_K_CSK kernel -- incremental derivative structure.

//! unpol: preamble=51 lines
//!   exc: shared=0, delta=51, outputs=1
//!   vxc: shared=51, delta=34, outputs=5
//!   fxc: shared=85, delta=113, outputs=15
//!   kxc: shared=198, delta=491, outputs=35
//!   lxc: shared=689, delta=1006, outputs=70
//! pol: preamble=86 lines
//!   exc: shared=0, delta=86, outputs=1
//!   vxc: shared=86, delta=84, outputs=10
//!   fxc: shared=170, delta=307, outputs=55
//!   kxc: shared=477, delta=1224, outputs=220
//!   lxc: shared=1701, delta=2647, outputs=715

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
