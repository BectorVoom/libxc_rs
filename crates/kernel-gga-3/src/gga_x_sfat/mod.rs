//! GGA_X_SFAT kernel -- incremental derivative structure.

//! unpol: preamble=85 lines
//!   exc: shared=0, delta=85, outputs=1
//!   vxc: shared=85, delta=99, outputs=3
//!   fxc: shared=184, delta=222, outputs=6
//!   kxc: shared=406, delta=281, outputs=10
//!   lxc: shared=687, delta=227, outputs=15
//! pol: preamble=165 lines
//!   exc: shared=0, delta=165, outputs=1
//!   vxc: shared=165, delta=281, outputs=6
//!   fxc: shared=446, delta=791, outputs=21
//!   kxc: shared=1237, delta=1339, outputs=56
//!   lxc: shared=2576, delta=1794, outputs=126

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
