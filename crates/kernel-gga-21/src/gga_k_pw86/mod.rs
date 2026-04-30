//! GGA_K_PW86 kernel -- incremental derivative structure.

//! unpol: preamble=41 lines
//!   exc: shared=0, delta=41, outputs=1
//!   vxc: shared=41, delta=21, outputs=3
//!   fxc: shared=62, delta=19, outputs=6
//!   kxc: shared=81, delta=31, outputs=10
//!   lxc: shared=112, delta=25, outputs=15
//! pol: preamble=69 lines
//!   exc: shared=0, delta=69, outputs=1
//!   vxc: shared=69, delta=60, outputs=6
//!   fxc: shared=129, delta=106, outputs=21
//!   kxc: shared=235, delta=233, outputs=56
//!   lxc: shared=468, delta=317, outputs=126

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
