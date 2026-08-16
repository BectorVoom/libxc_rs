//! LDA_C_ML1 kernel — per-functional subcrate (Phase 11 D-10).
//!
//! Output modules are enumerated below; each is either a single
//! `src/<output>.rs` file or a `src/<output>/` directory with a
//! nested-by-output partNN layout (D-04).
//!
//! Auto-emitted by tools/translate_v2/emit.py — do not hand-edit.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

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
