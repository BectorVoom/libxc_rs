//! MGGA_C_KCIS kernel -- incremental derivative structure.

//! unpol: preamble=205 lines
//!   exc: shared=0, delta=205, outputs=1
//!   vxc: shared=205, delta=294, outputs=5
//!   fxc: shared=499, delta=549, outputs=15
//!   kxc: shared=1048, delta=1024, outputs=35
//!   lxc: shared=2072, delta=1097, outputs=70
//! pol: preamble=385 lines
//!   exc: shared=0, delta=385, outputs=1
//!   vxc: shared=385, delta=920, outputs=10
//!   fxc: shared=1305, delta=3038, outputs=55
//!   kxc: shared=4343, delta=9978, outputs=220
//!   lxc: shared=14321, delta=18804, outputs=715

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
