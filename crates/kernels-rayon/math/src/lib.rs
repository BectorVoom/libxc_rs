//! Shared math primitives for the plain-Rust (rayon backend) kernels.
//!
//! Ported from `crates/kernels/math` with the `#[cube]` machinery removed and
//! the `<F: Float>` generics resolved to `f64`. Every function keeps the exact
//! expression sequence of its CubeCL counterpart, so kernels that call these
//! are bit-identical across the two backends.
//!
//! `select(c, a, b)` becomes `if c { a } else { b }`. That is faithful, not a
//! relaxation: callers always pass already-evaluated values, so both arms are
//! evaluated in either form.

/// Re-exported so generated SIMD kernels can name `f64x8` without taking a
/// second dependency. See `docs/perf/simd-kernels.md` for when a kernel is
/// emitted in SIMD form at all.
pub use wide;

pub mod constants;
pub mod piecewise;
pub mod powers;
pub mod simd;
pub(crate) mod simd_data;
pub mod bessel;
pub mod br89;
pub mod bspline;
pub mod dft_quantities;
pub mod erf;
pub mod expint_e1;
pub mod integrate;
pub mod lambert_w;
pub mod mbrxc;
pub mod polynomials;
pub mod special;
pub mod spin;

/// CubeCL forms of the primitives, for explicit lane-vectorised kernels.
/// Requires the `cubecl` feature; see `vector.rs` for why it is not default.
#[cfg(feature = "cubecl")]
pub mod vector;
