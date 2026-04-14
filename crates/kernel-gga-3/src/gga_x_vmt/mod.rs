//! GGA_X_VMT kernel -- incremental derivative structure.

//! unpol: preamble=36 lines
//!   exc: shared=0, delta=36, outputs=1
//!   vxc: shared=36, delta=34, outputs=3
//!   fxc: shared=70, delta=41, outputs=6
//!   kxc: shared=111, delta=57, outputs=10
//!   lxc: shared=168, delta=36, outputs=15
//! pol: preamble=61 lines
//!   exc: shared=0, delta=61, outputs=1
//!   vxc: shared=61, delta=70, outputs=6
//!   fxc: shared=131, delta=129, outputs=21
//!   kxc: shared=260, delta=244, outputs=56
//!   lxc: shared=504, delta=278, outputs=126

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
