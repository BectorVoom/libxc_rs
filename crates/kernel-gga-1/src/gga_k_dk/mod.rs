//! GGA_K_DK kernel -- incremental derivative structure.

//! unpol: preamble=55 lines
//!   exc: shared=0, delta=55, outputs=1
//!   vxc: shared=55, delta=34, outputs=3
//!   fxc: shared=89, delta=44, outputs=6
//!   kxc: shared=133, delta=69, outputs=10
//!   lxc: shared=202, delta=32, outputs=15
//! pol: preamble=100 lines
//!   exc: shared=0, delta=100, outputs=1
//!   vxc: shared=100, delta=90, outputs=6
//!   fxc: shared=190, delta=174, outputs=21
//!   kxc: shared=364, delta=387, outputs=56
//!   lxc: shared=751, delta=458, outputs=126

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
