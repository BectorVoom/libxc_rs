//! GGA_X_N12 kernel -- incremental derivative structure.

//! unpol: preamble=73 lines
//!   exc: shared=0, delta=73, outputs=1
//!   vxc: shared=73, delta=63, outputs=3
//!   fxc: shared=136, delta=79, outputs=6
//!   kxc: shared=215, delta=100, outputs=10
//!   lxc: shared=315, delta=57, outputs=15
//! pol: preamble=147 lines
//!   exc: shared=0, delta=147, outputs=1
//!   vxc: shared=147, delta=156, outputs=6
//!   fxc: shared=303, delta=250, outputs=21
//!   kxc: shared=553, delta=430, outputs=56
//!   lxc: shared=983, delta=570, outputs=126

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
