//! GGA_C_TCA kernel -- incremental derivative structure.

//! unpol: preamble=35 lines
//!   exc: shared=0, delta=35, outputs=1
//!   vxc: shared=35, delta=20, outputs=3
//!   fxc: shared=55, delta=47, outputs=6
//!   kxc: shared=102, delta=83, outputs=10
//!   lxc: shared=185, delta=65, outputs=15
//! pol: preamble=51 lines
//!   exc: shared=0, delta=51, outputs=1
//!   vxc: shared=51, delta=48, outputs=6
//!   fxc: shared=99, delta=150, outputs=21
//!   kxc: shared=249, delta=408, outputs=56
//!   lxc: shared=657, delta=979, outputs=126

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
