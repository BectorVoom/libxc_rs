//! GGA_X_PBEPOW kernel -- incremental derivative structure.

//! unpol: preamble=37 lines
//!   exc: shared=0, delta=37, outputs=1
//!   vxc: shared=37, delta=29, outputs=3
//!   fxc: shared=66, delta=43, outputs=6
//!   kxc: shared=109, delta=63, outputs=10
//!   lxc: shared=172, delta=28, outputs=15
//! pol: preamble=61 lines
//!   exc: shared=0, delta=61, outputs=1
//!   vxc: shared=61, delta=76, outputs=6
//!   fxc: shared=137, delta=140, outputs=21
//!   kxc: shared=277, delta=261, outputs=56
//!   lxc: shared=538, delta=267, outputs=126

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
