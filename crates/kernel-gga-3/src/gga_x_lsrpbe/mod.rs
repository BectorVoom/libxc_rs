//! GGA_X_LSRPBE kernel -- incremental derivative structure.

//! unpol: preamble=32 lines
//!   exc: shared=0, delta=32, outputs=1
//!   vxc: shared=32, delta=15, outputs=3
//!   fxc: shared=47, delta=38, outputs=6
//!   kxc: shared=85, delta=37, outputs=10
//!   lxc: shared=122, delta=29, outputs=15
//! pol: preamble=57 lines
//!   exc: shared=0, delta=57, outputs=1
//!   vxc: shared=57, delta=44, outputs=6
//!   fxc: shared=101, delta=114, outputs=21
//!   kxc: shared=215, delta=198, outputs=56
//!   lxc: shared=413, delta=265, outputs=126

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
