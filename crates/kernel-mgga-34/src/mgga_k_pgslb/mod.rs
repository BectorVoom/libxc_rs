//! MGGA_K_PGSLB kernel -- incremental derivative structure.

//! unpol: preamble=42 lines
//!   exc: shared=0, delta=42, outputs=1
//!   vxc: shared=42, delta=15, outputs=5
//!   fxc: shared=57, delta=35, outputs=15
//!   kxc: shared=92, delta=45, outputs=35
//!   lxc: shared=137, delta=51, outputs=70
//! pol: preamble=68 lines
//!   exc: shared=0, delta=68, outputs=1
//!   vxc: shared=68, delta=57, outputs=10
//!   fxc: shared=125, delta=149, outputs=55
//!   kxc: shared=274, delta=362, outputs=220
//!   lxc: shared=636, delta=715, outputs=715

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
