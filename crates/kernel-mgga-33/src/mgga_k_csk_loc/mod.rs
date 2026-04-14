//! MGGA_K_CSK_LOC kernel -- incremental derivative structure.

//! unpol: preamble=57 lines
//!   exc: shared=0, delta=57, outputs=1
//!   vxc: shared=57, delta=37, outputs=5
//!   fxc: shared=94, delta=112, outputs=15
//!   kxc: shared=206, delta=484, outputs=35
//!   lxc: shared=690, delta=1016, outputs=70
//! pol: preamble=93 lines
//!   exc: shared=0, delta=93, outputs=1
//!   vxc: shared=93, delta=88, outputs=10
//!   fxc: shared=181, delta=303, outputs=55
//!   kxc: shared=484, delta=1206, outputs=220
//!   lxc: shared=1690, delta=2661, outputs=715

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
