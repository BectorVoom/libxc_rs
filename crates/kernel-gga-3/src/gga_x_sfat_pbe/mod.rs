//! GGA_X_SFAT_PBE kernel -- incremental derivative structure.

//! unpol: preamble=80 lines
//!   exc: shared=0, delta=80, outputs=1
//!   vxc: shared=80, delta=93, outputs=3
//!   fxc: shared=173, delta=186, outputs=6
//!   kxc: shared=359, delta=254, outputs=10
//!   lxc: shared=613, delta=249, outputs=15
//! pol: preamble=157 lines
//!   exc: shared=0, delta=157, outputs=1
//!   vxc: shared=157, delta=274, outputs=6
//!   fxc: shared=431, delta=772, outputs=21
//!   kxc: shared=1203, delta=1300, outputs=56
//!   lxc: shared=2503, delta=2019, outputs=126

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
