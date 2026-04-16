//! GGA_C_OP_B88 kernel -- incremental derivative structure.

//! unpol: preamble=63 lines
//!   exc: shared=0, delta=63, outputs=1
//!   vxc: shared=63, delta=49, outputs=3
//!   fxc: shared=112, delta=113, outputs=6
//!   kxc: shared=225, delta=209, outputs=10
//!   lxc: shared=434, delta=167, outputs=15
//! pol: preamble=84 lines
//!   exc: shared=0, delta=84, outputs=1
//!   vxc: shared=84, delta=114, outputs=6
//!   fxc: shared=198, delta=351, outputs=21
//!   kxc: shared=549, delta=996, outputs=56
//!   lxc: shared=1545, delta=1663, outputs=126

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
