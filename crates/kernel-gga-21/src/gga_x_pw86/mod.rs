//! GGA_X_PW86 kernel -- incremental derivative structure.

//! unpol: preamble=44 lines
//!   exc: shared=0, delta=44, outputs=1
//!   vxc: shared=44, delta=22, outputs=3
//!   fxc: shared=66, delta=21, outputs=6
//!   kxc: shared=87, delta=31, outputs=10
//!   lxc: shared=118, delta=25, outputs=15
//! pol: preamble=72 lines
//!   exc: shared=0, delta=72, outputs=1
//!   vxc: shared=72, delta=63, outputs=6
//!   fxc: shared=135, delta=108, outputs=21
//!   kxc: shared=243, delta=236, outputs=56
//!   lxc: shared=479, delta=317, outputs=126

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
