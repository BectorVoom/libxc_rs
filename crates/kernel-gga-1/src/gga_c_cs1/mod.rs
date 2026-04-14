//! GGA_C_CS1 kernel -- incremental derivative structure.

//! unpol: preamble=32 lines
//!   exc: shared=0, delta=32, outputs=1
//!   vxc: shared=32, delta=35, outputs=3
//!   fxc: shared=67, delta=58, outputs=6
//!   kxc: shared=125, delta=91, outputs=10
//!   lxc: shared=216, delta=26, outputs=15
//! pol: preamble=66 lines
//!   exc: shared=0, delta=66, outputs=1
//!   vxc: shared=66, delta=112, outputs=6
//!   fxc: shared=178, delta=250, outputs=21
//!   kxc: shared=428, delta=526, outputs=56
//!   lxc: shared=954, delta=403, outputs=126

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
