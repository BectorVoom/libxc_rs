//! GGA_X_VMT84 kernel -- incremental derivative structure.

//! unpol: preamble=52 lines
//!   exc: shared=0, delta=52, outputs=1
//!   vxc: shared=52, delta=34, outputs=3
//!   fxc: shared=86, delta=61, outputs=6
//!   kxc: shared=147, delta=84, outputs=10
//!   lxc: shared=231, delta=51, outputs=15
//! pol: preamble=84 lines
//!   exc: shared=0, delta=84, outputs=1
//!   vxc: shared=84, delta=74, outputs=6
//!   fxc: shared=158, delta=155, outputs=21
//!   kxc: shared=313, delta=290, outputs=56
//!   lxc: shared=603, delta=304, outputs=126

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
