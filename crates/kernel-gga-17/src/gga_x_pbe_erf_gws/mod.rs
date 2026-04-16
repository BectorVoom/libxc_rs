//! GGA_X_PBE_ERF_GWS kernel -- incremental derivative structure.

//! unpol: preamble=121 lines
//!   exc: shared=0, delta=121, outputs=1
//!   vxc: shared=121, delta=93, outputs=3
//!   fxc: shared=214, delta=150, outputs=6
//!   kxc: shared=364, delta=246, outputs=10
//!   lxc: shared=610, delta=100, outputs=15
//! pol: preamble=181 lines
//!   exc: shared=0, delta=181, outputs=1
//!   vxc: shared=181, delta=177, outputs=6
//!   fxc: shared=358, delta=290, outputs=21
//!   kxc: shared=648, delta=480, outputs=56
//!   lxc: shared=1128, delta=257, outputs=126

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
