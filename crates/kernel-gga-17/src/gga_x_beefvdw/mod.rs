//! GGA_X_BEEFVDW kernel -- incremental derivative structure.

//! unpol: preamble=62 lines
//!   exc: shared=0, delta=62, outputs=1
//!   vxc: shared=62, delta=60, outputs=3
//!   fxc: shared=122, delta=172, outputs=6
//!   kxc: shared=294, delta=216, outputs=10
//!   lxc: shared=510, delta=106, outputs=15
//! pol: preamble=115 lines
//!   exc: shared=0, delta=115, outputs=1
//!   vxc: shared=115, delta=134, outputs=6
//!   fxc: shared=249, delta=420, outputs=21
//!   kxc: shared=669, delta=563, outputs=56
//!   lxc: shared=1232, delta=419, outputs=126

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
