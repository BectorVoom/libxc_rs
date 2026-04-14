//! GGA_C_CHACHIYO kernel -- incremental derivative structure.

//! unpol: preamble=36 lines
//!   exc: shared=0, delta=36, outputs=1
//!   vxc: shared=36, delta=29, outputs=3
//!   fxc: shared=65, delta=55, outputs=6
//!   kxc: shared=120, delta=68, outputs=10
//!   lxc: shared=188, delta=42, outputs=15
//! pol: preamble=52 lines
//!   exc: shared=0, delta=52, outputs=1
//!   vxc: shared=52, delta=56, outputs=6
//!   fxc: shared=108, delta=136, outputs=21
//!   kxc: shared=244, delta=281, outputs=56
//!   lxc: shared=525, delta=496, outputs=126

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
