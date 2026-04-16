//! GGA_X_NCAP kernel -- incremental derivative structure.

//! unpol: preamble=43 lines
//!   exc: shared=0, delta=43, outputs=1
//!   vxc: shared=43, delta=73, outputs=3
//!   fxc: shared=116, delta=143, outputs=6
//!   kxc: shared=259, delta=318, outputs=10
//!   lxc: shared=577, delta=296, outputs=15
//! pol: preamble=76 lines
//!   exc: shared=0, delta=76, outputs=1
//!   vxc: shared=76, delta=131, outputs=6
//!   fxc: shared=207, delta=291, outputs=21
//!   kxc: shared=498, delta=713, outputs=56
//!   lxc: shared=1211, delta=854, outputs=126

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
