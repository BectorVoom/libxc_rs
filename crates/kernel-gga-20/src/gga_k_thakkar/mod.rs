//! GGA_K_THAKKAR kernel -- incremental derivative structure.

//! unpol: preamble=36 lines
//!   exc: shared=0, delta=36, outputs=1
//!   vxc: shared=36, delta=29, outputs=3
//!   fxc: shared=65, delta=55, outputs=6
//!   kxc: shared=120, delta=76, outputs=10
//!   lxc: shared=196, delta=38, outputs=15
//! pol: preamble=67 lines
//!   exc: shared=0, delta=67, outputs=1
//!   vxc: shared=67, delta=74, outputs=6
//!   fxc: shared=141, delta=161, outputs=21
//!   kxc: shared=302, delta=279, outputs=56
//!   lxc: shared=581, delta=295, outputs=126

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
