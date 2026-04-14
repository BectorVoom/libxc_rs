//! MGGA_XC_ZLP kernel -- incremental derivative structure.

//! unpol: preamble=22 lines
//!   exc: shared=0, delta=22, outputs=1
//!   vxc: shared=22, delta=16, outputs=5
//!   fxc: shared=38, delta=23, outputs=15
//!   kxc: shared=61, delta=34, outputs=35
//!   lxc: shared=95, delta=37, outputs=70
//! pol: preamble=41 lines
//!   exc: shared=0, delta=41, outputs=1
//!   vxc: shared=41, delta=46, outputs=10
//!   fxc: shared=87, delta=120, outputs=55
//!   kxc: shared=207, delta=316, outputs=220
//!   lxc: shared=523, delta=662, outputs=715

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
