//! MGGA_C_CC kernel -- incremental derivative structure.

//! unpol: preamble=30 lines
//!   exc: shared=0, delta=30, outputs=1
//!   vxc: shared=30, delta=32, outputs=5
//!   fxc: shared=62, delta=64, outputs=15
//!   kxc: shared=126, delta=89, outputs=35
//!   lxc: shared=215, delta=70, outputs=70
//! pol: preamble=80 lines
//!   exc: shared=0, delta=80, outputs=1
//!   vxc: shared=80, delta=116, outputs=10
//!   fxc: shared=196, delta=334, outputs=55
//!   kxc: shared=530, delta=894, outputs=220
//!   lxc: shared=1424, delta=1692, outputs=715

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
