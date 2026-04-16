//! GGA_C_OP_G96 kernel -- incremental derivative structure.

//! unpol: preamble=56 lines
//!   exc: shared=0, delta=56, outputs=1
//!   vxc: shared=56, delta=42, outputs=3
//!   fxc: shared=98, delta=97, outputs=6
//!   kxc: shared=195, delta=190, outputs=10
//!   lxc: shared=385, delta=179, outputs=15
//! pol: preamble=73 lines
//!   exc: shared=0, delta=73, outputs=1
//!   vxc: shared=73, delta=101, outputs=6
//!   fxc: shared=174, delta=331, outputs=21
//!   kxc: shared=505, delta=959, outputs=56
//!   lxc: shared=1464, delta=1679, outputs=126

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
