//! CubeCL forms of the math primitives, and kernels launched through a client.
//!
//! Feature-gated behind `cubecl`, **off by default**, and deliberately so:
//! `cubecl` pulls ~235 transitive crates, and this crate is a dependency of
//! all 266 kernel crates. Turning it on unconditionally would put that cost
//! into every build of the production path, which is exactly what
//! `crates/libxc-eval/Cargo.toml` records going wrong before.
//!
//! # Why the primitives exist twice
//!
//! A `#[cube]` function is not compiled as Rust — it is parsed into CubeCL's
//! IR and a kernel is generated from that. A plain Rust function therefore
//! cannot be called from one: the macro tries to resolve the call as a CubeCL
//! item and rustc reports `is not a crate or module (E0433)`. The scalar
//! helpers in the sibling modules are unreachable from a kernel, so the
//! primitives have to exist a second time in this form.
//!
//! # Running one
//!
//! Kernels are launched through a `ComputeClient`, and `run_scalar` /
//! `run_vectorised` below are complete worked examples:
//!
//! ```text
//! let client = CpuRuntime::client(&CpuDevice::default());
//! let h      = client.create(Bytes::from_elems(rho.to_vec()));   // upload
//! kernel::launch::<f64, CpuRuntime>(&client, count, dim, args..); // enqueue
//! let bytes  = client.read(vec![out_handle]);                     // read back
//! ```
//!
//! Both are covered by tests: the kernel's output matches the same formula in
//! plain Rust, and launching it at lane widths 1, 2 and 4 gives the same
//! answer.
//!
//! # Vectorisation, and the trait wall
//!
//! `Vector<F, N>` is CubeCL's SIMD lane vector, with `N` chosen **at launch**
//! rather than at compile time — it is an extra argument to `launch`, after
//! `CubeDim` — so one kernel body specialises to whatever width is asked for.
//!
//! The catch, and the reason the vector kernel below cannot simply call the
//! scalar helpers: **`Vector<F, N>` does not implement `Float`.** It implements
//! the individual operation traits — `Powf`, `Sqrt`, `Exp`, `Log`, `Erf`,
//! `Tanh`, `ArcTan`, `ArcTanh`, `InverseSqrt`, `Hypot`, `Dot` — but not the
//! `Float` supertrait that bundles them, so a helper written as
//! `fn f<F: Float>(..)` is scalar-only. A primitive that must serve both widths
//! has to be bounded on the specific traits it uses, or written twice.
//!
//! Two further consequences worth knowing before writing more of these:
//!
//! * **`select` is scalar-conditioned.** `select(cond: bool, ..)` cannot say
//!   "these lanes took the branch and those did not". The lane-mask form is
//!   `select_many(cond: Vector<bool, N>, ..)`, which is what
//!   [`piecewise3_many`] wraps — and a lane mask is what a comparison on a
//!   `Vector` produces (`r.less_equal(thr)`).
//! * **There is no `cbrt`.** [`pow_1_3`] is `powf(x, 1/3)`, which is *not*
//!   bit-identical to the scalar [`crate::powers::cbrt_f64`]. Since `POW_1_3`
//!   is the third most common call in the maple2c sources (13,478 sites), any
//!   real kernel path here would need a hand-written cube root first.
//!
//! Everything else the XC kernels need is present, contrary to what the
//! CubeCL manual's algebra table lists: `atan`, `atan2`, `tanh` and `erf` all
//! exist on `Float` and on `Vector`.

use cubecl::prelude::*;

// ---------------------------------------------------------------------------
// Primitives
// ---------------------------------------------------------------------------

/// Branch-free 2-way select — libxc's `my_piecewise3(c, x1, x2)`.
///
/// Both arms are evaluated, matching the C macro and the scalar
/// [`crate::piecewise::piecewise3`]. `select` is used rather than an `if`
/// expression: CubeCL's own guidance is that `let x = if c { a } else { b }`
/// can fail to lower, and on a `Vector` the condition is a lane mask, so this
/// is a blend rather than a branch.
#[cube]
pub fn piecewise3<F: Float>(cond: bool, val_true: F, val_false: F) -> F {
    select(cond, val_true, val_false)
}

/// Lane-wise `my_piecewise3` for a `Vector`.
///
/// `select` takes a single `bool`, so it cannot express "these lanes took the
/// threshold branch and those did not". `select_many` takes a
/// `Vector<bool, N>` mask instead and blends per lane, which is what makes a
/// screened grid point cost its lane nothing while its neighbours compute.
#[cube]
pub fn piecewise3_many<F: Scalar, N: Size>(
    cond: Vector<bool, N>,
    val_true: Vector<F, N>,
    val_false: Vector<F, N>,
) -> Vector<F, N> {
    select_many::<F, N>(cond, val_true, val_false)
}

/// Branch-free 3-way select — libxc's `my_piecewise5(c1, x1, c2, x2, x3)`.
#[cube]
pub fn piecewise5<F: Float>(c1: bool, v1: F, c2: bool, v2: F, v_else: F) -> F {
    select(c1, v1, select(c2, v2, v_else))
}

/// Heaviside step — libxc's `Heaviside(x)`.
#[cube]
#[allow(non_snake_case)]
pub fn Heaviside<F: Float>(x: F) -> F {
    select(x >= F::from_int(0), F::from_int(1), F::from_int(0))
}

/// `x^2`, as libxc's `POW_2`.
#[cube]
pub fn pow_2<F: Float>(x: F) -> F {
    x * x
}

/// `x^3`, as libxc's `POW_3`.
#[cube]
pub fn pow_3<F: Float>(x: F) -> F {
    x * x * x
}

/// `sqrt(sqrt(x))`, as libxc's `POW_1_4`.
#[cube]
pub fn pow_1_4<F: Float>(x: F) -> F {
    F::sqrt(F::sqrt(x))
}

/// `x * sqrt(x)`, as libxc's `POW_3_2`.
#[cube]
pub fn pow_3_2<F: Float>(x: F) -> F {
    x * F::sqrt(x)
}

/// Cube root, as libxc's `POW_1_3`.
///
/// **Not bit-identical to [`crate::powers::cbrt_f64`].** The scalar version
/// reduces the exponent by hand and finishes with a Newton step, staying inside
/// 1 ulp; CubeCL exposes no `cbrt`, so this is `powf(x, 1/3)`, whose error is
/// whatever the backend's `pow` gives. It also does not accept negative input,
/// which the scalar one does.
#[cube]
pub fn pow_1_3<F: Float>(x: F) -> F {
    F::powf(x, F::cast_from(1.0_f64 / 3.0_f64))
}

/// `cbrt(x)^2`, as libxc's `POW_2_3`.
#[cube]
pub fn pow_2_3<F: Float>(x: F) -> F {
    let c = pow_1_3::<F>(x);
    c * c
}

/// `x * cbrt(x)`, as libxc's `POW_4_3`.
#[cube]
pub fn pow_4_3<F: Float>(x: F) -> F {
    x * pow_1_3::<F>(x)
}

/// `x * cbrt(x)^2`, as libxc's `POW_5_3`.
#[cube]
pub fn pow_5_3<F: Float>(x: F) -> F {
    let c = pow_1_3::<F>(x);
    x * c * c
}

/// `x^2 * cbrt(x)`, as libxc's `POW_7_3`.
#[cube]
pub fn pow_7_3<F: Float>(x: F) -> F {
    x * x * pow_1_3::<F>(x)
}

// ---------------------------------------------------------------------------
// Launchable kernels
// ---------------------------------------------------------------------------

/// Slater exchange energy density per particle,
/// `zk = -(3/4)(3/pi)^(1/3) rho^(1/3)`, with the low-density branch libxc
/// applies. This is `lda_x` exc unpolarized, the simplest real functional in
/// the set.
///
/// Scalar form: one grid point per unit. `ABSOLUTE_POS` is the unit index and
/// `rho.len()` the element count.
///
/// The `CubeElement` bound is required by `#[cube(launch)]`, not by the body:
/// a launchable kernel taking a scalar argument needs `F` to be a type the
/// host can hand across as a `ScalarArg`.
#[cube(launch)]
pub fn lda_x_exc_kernel<F: Float + CubeElement>(
    rho: &Array<F>,
    zk: &mut Array<F>,
    dens_threshold: F,
) {
    let i = ABSOLUTE_POS;
    if i < rho.len() {
        let r = rho[i];
        let c = F::cast_from(LDA_X_C);
        zk[i] = piecewise3::<F>(r <= dens_threshold, F::from_int(0), c * pow_1_3::<F>(r));
    }
}

/// The same functional over `Vector<F, N>` — `N` lanes per unit, with `N`
/// chosen at launch rather than compile time.
///
/// The body is identical to the scalar one apart from two things that the
/// lane type forces:
///
/// * the scalar threshold has to be broadcast with `Vector::new` before it can
///   be compared against a vector, and
/// * `rho.len()` is now the number of *vectors*, so the launch geometry is
///   `points / N` units.
///
/// `piecewise3` needs no change: on a vector the comparison yields a lane mask
/// and `select` becomes a blend, which is exactly the branch-free semantics
/// libxc's `my_piecewise3` macro has.
#[cube(launch)]
pub fn lda_x_exc_vec_kernel<F: Float + CubeElement, N: Size>(
    rho: &Array<Vector<F, N>>,
    zk: &mut Array<Vector<F, N>>,
    dens_threshold: F,
) {
    let i = ABSOLUTE_POS;
    if i < rho.len() {
        let r = rho[i];
        // A scalar has to be broadcast before it can meet a vector.
        let c = Vector::<F, N>::new(F::cast_from(LDA_X_C));
        let thr = Vector::<F, N>::new(dens_threshold);
        let zero = Vector::<F, N>::new(F::from_int(0));
        // `Vector` implements Powf/Sqrt/Log/Exp/Erf/ArcTan/Tanh individually,
        // but not the `Float` trait itself, so the scalar helpers above cannot
        // be instantiated at this type; the cube root is written out here.
        let third = Vector::<F, N>::new(F::cast_from(1.0_f64 / 3.0_f64));
        let e = c * Powf::powf(r, third);
        zk[i] = piecewise3_many::<F, N>(r.less_equal(thr), zero, e);
    }
}

// ---------------------------------------------------------------------------
// Host-side launch helpers
// ---------------------------------------------------------------------------

/// `-(3/4) * (3/pi)^(1/3)`, the Slater exchange coefficient.
pub const LDA_X_C: f64 = -0.7385587663820224_f64;

mod host {
    use super::*;
    use cubecl::cpu::{CpuDevice, CpuRuntime};

    /// Reference form of what the kernels compute, in plain Rust.
    pub fn lda_x_exc_reference(rho: &[f64], dens_threshold: f64) -> Vec<f64> {
        rho.iter()
            .map(|&r| if r <= dens_threshold { 0.0 } else { LDA_X_C * r.powf(1.0 / 3.0) })
            .collect()
    }

    fn read_f64(
        client: &cubecl::client::ComputeClient<CpuRuntime>,
        handle: cubecl::server::Handle,
        n: usize,
    ) -> Vec<f64> {
        let bytes = client.read(vec![handle]);
        bytemuck::cast_slice::<u8, f64>(&bytes[0])[..n].to_vec()
    }

    /// Launch the scalar kernel through a CubeCL client on the CPU backend.
    pub fn run_scalar(rho: &[f64], dens_threshold: f64) -> Vec<f64> {
        let device = CpuDevice::default();
        let client = CpuRuntime::client(&device);
        let n = rho.len();
        let rho_h = client.create(cubecl::bytes::Bytes::from_elems(rho.to_vec()));
        let zk_h = client.create(cubecl::bytes::Bytes::from_elems(vec![0.0f64; n]));

        lda_x_exc_kernel::launch::<f64, CpuRuntime>(
            &client,
            CubeCount::Static(1, 1, 1),
            CubeDim { x: n as u32, y: 1, z: 1 },
            unsafe { ArrayArg::from_raw_parts(rho_h, n) },
            unsafe { ArrayArg::from_raw_parts(zk_h.clone(), n) },
            dens_threshold,
        );
        read_f64(&client, zk_h, n)
    }

    /// Launch the lane-vectorised kernel at width `n_lanes`.
    ///
    /// The vectorisation factor is a *launch* argument — it appears after
    /// `CubeDim` and before the buffers — which is what lets one kernel body
    /// serve every width. Buffer lengths stay in scalars; CubeCL derives the
    /// vector indexing from the factor, so the grid is `len / n_lanes` units.
    pub fn run_vectorised(rho: &[f64], dens_threshold: f64, n_lanes: u8) -> Vec<f64> {
        assert!(
            rho.len() % n_lanes as usize == 0,
            "grid length {} is not a multiple of the lane count {n_lanes}",
            rho.len()
        );
        let device = CpuDevice::default();
        let client = CpuRuntime::client(&device);
        let n = rho.len();
        let rho_h = client.create(cubecl::bytes::Bytes::from_elems(rho.to_vec()));
        let zk_h = client.create(cubecl::bytes::Bytes::from_elems(vec![0.0f64; n]));

        lda_x_exc_vec_kernel::launch::<f64, CpuRuntime>(
            &client,
            CubeCount::Static(1, 1, 1),
            CubeDim { x: (n / n_lanes as usize) as u32, y: 1, z: 1 },
            n_lanes as usize,
            unsafe { ArrayArg::from_raw_parts(rho_h, n) },
            unsafe { ArrayArg::from_raw_parts(zk_h.clone(), n) },
            dens_threshold,
        );
        read_f64(&client, zk_h, n)
    }
}

pub use host::{lda_x_exc_reference, run_scalar, run_vectorised};

#[cfg(test)]
mod tests {
    use super::*;

    fn grid(n: usize) -> Vec<f64> {
        // Spans the physical range and includes points under the threshold, so
        // the `piecewise3` branch is exercised in both directions.
        (0..n)
            .map(|i| if i % 8 == 0 { 1e-30 } else { 1e-4 * (i as f64 + 1.0) })
            .collect()
    }

    fn assert_close(got: &[f64], want: &[f64], what: &str) {
        for (i, (g, w)) in got.iter().zip(want).enumerate() {
            assert!(
                (g - w).abs() <= 1e-13 * w.abs().max(1.0),
                "{what}, point {i}: {g} vs {w}"
            );
        }
    }

    /// The kernel runs through the client and agrees with plain Rust.
    #[test]
    fn scalar_kernel_matches_reference() {
        let rho = grid(64);
        assert_close(&run_scalar(&rho, 1e-15), &lda_x_exc_reference(&rho, 1e-15), "scalar");
    }

    /// The same body launched at several lane widths gives the same answer.
    /// This is the property that makes `Vector<F, N>` worth having: the width
    /// is a launch parameter, not a second kernel.
    #[test]
    fn lane_width_does_not_change_the_result() {
        let rho = grid(64);
        let want = lda_x_exc_reference(&rho, 1e-15);
        for w in [1u8, 2, 4] {
            assert_close(&run_vectorised(&rho, 1e-15, w), &want, &format!("lanes={w}"));
        }
    }
}
