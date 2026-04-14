//! GGA_X_B88 kernel -- incremental derivative structure.

//! unpol: preamble=38 lines
//!   exc: shared=0, delta=38, outputs=1
//!   vxc: shared=38, delta=27, outputs=3
//!   fxc: shared=65, delta=44, outputs=6
//!   kxc: shared=109, delta=69, outputs=10
//!   lxc: shared=178, delta=31, outputs=15
//! pol: preamble=62 lines
//!   exc: shared=0, delta=62, outputs=1
//!   vxc: shared=62, delta=71, outputs=6
//!   fxc: shared=133, delta=138, outputs=21
//!   kxc: shared=271, delta=251, outputs=56
//!   lxc: shared=522, delta=263, outputs=126

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
