//! B-spline evaluation for CubeCL kernels.
//!
//! Specialized for hyb_gga_xc_case21 functional (k=3, Nsp=10).
//! Knots are precomputed constants: knots[i] = -3/7 + i/7 for i=0..13.
//!
//! Matches the libxc C original (`xc_bspline` in `util.c`) control flow: uses
//! `if/else` guards to skip computation outside the support interval and to
//! dispatch only the requested derivative order. CubeCL 0.9.0 does not support
//! `return` in `#[cube]` functions, so we use mutable result + `if/else` guards
//! instead of early returns.
//!
//! Mixed-precision signature note (Rule 7): `knot`, `bspline_k3_eval`,
//! `case21_xbspline`, and `case21_cbspline` take `u32` integer index parameters
//! (`idx`/`i`/`ider`) alongside their generic `<F: Float>` float parameters.
//! The float portion of each signature is fully generic.

use cubecl::prelude::*;

/// Get knot value by index (inlined constant lookup for CubeCL).
/// Case21: k=3, Nsp=10, knots[i] = (-3 + i) / 7
#[cube]
fn knot<F: Float>(idx: u32) -> F {
    let i = F::cast_from(idx);
    (i - F::new(3.0)) / F::new(7.0)
}

/// Safe division: returns 0 if denominator is 0, otherwise a/b.
#[cube]
fn safe_div<F: Float>(a: F, b: F) -> F {
    select(b == F::new(0.0), F::new(0.0), a / b)
}

/// Evaluate a single B-spline basis function N_{i,3}(u) for derivative order `ider`.
///
/// For k=3, supports ider=0..3 (ider>=4 returns 0 since maxk=min(4,3)=3).
/// Uses `if/else` guards matching libxc's `xc_bspline` control flow: skips
/// computation outside support, dispatches only the needed derivative order.
#[cube]
fn bspline_k3_eval<F: Float>(i: u32, u: F, ider: u32) -> F {
    let ki0 = knot::<F>(i);
    let ki1 = knot::<F>(i + 1);
    let ki2 = knot::<F>(i + 2);
    let ki3 = knot::<F>(i + 3);
    let ki4 = knot::<F>(i + 4);

    let mut result = F::new(0.0);

    // Guard: only compute if u is in support [knots[i], knots[i+4])
    if u >= ki0 {
        if u < ki4 {
            // Degree 0: piecewise constants (always needed)
            let n0_0 = select(u >= ki0, select(u < ki1, F::new(1.0), F::new(0.0)), F::new(0.0));
            let n0_1 = select(u >= ki1, select(u < ki2, F::new(1.0), F::new(0.0)), F::new(0.0));
            let n0_2 = select(u >= ki2, select(u < ki3, F::new(1.0), F::new(0.0)), F::new(0.0));
            let n0_3 = select(u >= ki3, select(u < ki4, F::new(1.0), F::new(0.0)), F::new(0.0));

            if ider == 3 {
                // Derivative order 3: only needs N[0] values, 3 triangular passes
                let d3_a0 = safe_div::<F>(n0_0, ki1 - ki0);
                let d3_a1 = safe_div::<F>(n0_1, ki2 - ki1);
                let d3_a2 = safe_div::<F>(n0_2, ki3 - ki2);
                let d3_a3 = safe_div::<F>(n0_3, ki4 - ki3);
                let d3_b0 = d3_a0 - d3_a1;
                let d3_b1 = d3_a1 - d3_a2;
                let d3_b2 = d3_a2 - d3_a3;
                let d3_c0 = safe_div::<F>(d3_b0, ki2 - ki0);
                let d3_c1 = safe_div::<F>(d3_b1, ki3 - ki1);
                let d3_c2 = safe_div::<F>(d3_b2, ki4 - ki2);
                let d3_d0 = F::new(2.0) * (d3_c0 - d3_c1);
                let d3_d1 = F::new(2.0) * (d3_c1 - d3_c2);
                let d3_e0 = safe_div::<F>(d3_d0, ki3 - ki0);
                let d3_e1 = safe_div::<F>(d3_d1, ki4 - ki1);
                result = F::new(3.0) * (d3_e0 - d3_e1);
            } else {
                // Degree 1: needed for ider <= 2
                let n1_0 = safe_div::<F>((u - ki0) * n0_0, ki1 - ki0)
                         + safe_div::<F>((ki2 - u) * n0_1, ki2 - ki1);
                let n1_1 = safe_div::<F>((u - ki1) * n0_1, ki2 - ki1)
                         + safe_div::<F>((ki3 - u) * n0_2, ki3 - ki2);
                let n1_2 = safe_div::<F>((u - ki2) * n0_2, ki3 - ki2)
                         + safe_div::<F>((ki4 - u) * n0_3, ki4 - ki3);

                if ider == 2 {
                    // Derivative order 2: needs N[1] values, 2 triangular passes
                    let d2_a0 = safe_div::<F>(n1_0, ki2 - ki0);
                    let d2_a1 = safe_div::<F>(n1_1, ki3 - ki1);
                    let d2_a2 = safe_div::<F>(n1_2, ki4 - ki2);
                    let d2_b0 = F::new(2.0) * (d2_a0 - d2_a1);
                    let d2_b1 = F::new(2.0) * (d2_a1 - d2_a2);
                    let d2_c0 = safe_div::<F>(d2_b0, ki3 - ki0);
                    let d2_c1 = safe_div::<F>(d2_b1, ki4 - ki1);
                    result = F::new(3.0) * (d2_c0 - d2_c1);
                } else {
                    // Degree 2: needed for ider <= 1
                    let n2_0 = safe_div::<F>((u - ki0) * n1_0, ki2 - ki0)
                             + safe_div::<F>((ki3 - u) * n1_1, ki3 - ki1);
                    let n2_1 = safe_div::<F>((u - ki1) * n1_1, ki3 - ki1)
                             + safe_div::<F>((ki4 - u) * n1_2, ki4 - ki2);

                    if ider == 1 {
                        // Derivative order 1: needs N[2] values
                        let d1_s0 = safe_div::<F>(n2_0, ki3 - ki0);
                        let d1_s1 = safe_div::<F>(n2_1, ki4 - ki1);
                        result = F::new(3.0) * (d1_s0 - d1_s1);
                    } else {
                        // ider == 0: function value, needs full N[3]
                        result = safe_div::<F>((u - ki0) * n2_0, ki3 - ki0)
                               + safe_div::<F>((ki4 - u) * n2_1, ki4 - ki1);
                    }
                }
            }
        }
    }

    result
}

/// Evaluate case21 exchange B-spline: sum_i cx[i] * B_{i,3}(u, ider)
///
/// cx_0..cx_9 are the 10 exchange enhancement coefficients.
#[cube]
#[allow(clippy::too_many_arguments)]
pub fn case21_xbspline<F: Float>(
    u: F, ider: u32,
    cx_0: F, cx_1: F, cx_2: F, cx_3: F, cx_4: F,
    cx_5: F, cx_6: F, cx_7: F, cx_8: F, cx_9: F,
) -> F {
    cx_0 * bspline_k3_eval::<F>(0, u, ider)
        + cx_1 * bspline_k3_eval::<F>(1, u, ider)
        + cx_2 * bspline_k3_eval::<F>(2, u, ider)
        + cx_3 * bspline_k3_eval::<F>(3, u, ider)
        + cx_4 * bspline_k3_eval::<F>(4, u, ider)
        + cx_5 * bspline_k3_eval::<F>(5, u, ider)
        + cx_6 * bspline_k3_eval::<F>(6, u, ider)
        + cx_7 * bspline_k3_eval::<F>(7, u, ider)
        + cx_8 * bspline_k3_eval::<F>(8, u, ider)
        + cx_9 * bspline_k3_eval::<F>(9, u, ider)
}

/// Evaluate case21 correlation B-spline: sum_i cc[i] * B_{i,3}(u, ider)
///
/// cc_0..cc_9 are the 10 correlation enhancement coefficients.
#[cube]
#[allow(clippy::too_many_arguments)]
pub fn case21_cbspline<F: Float>(
    u: F, ider: u32,
    cc_0: F, cc_1: F, cc_2: F, cc_3: F, cc_4: F,
    cc_5: F, cc_6: F, cc_7: F, cc_8: F, cc_9: F,
) -> F {
    cc_0 * bspline_k3_eval::<F>(0, u, ider)
        + cc_1 * bspline_k3_eval::<F>(1, u, ider)
        + cc_2 * bspline_k3_eval::<F>(2, u, ider)
        + cc_3 * bspline_k3_eval::<F>(3, u, ider)
        + cc_4 * bspline_k3_eval::<F>(4, u, ider)
        + cc_5 * bspline_k3_eval::<F>(5, u, ider)
        + cc_6 * bspline_k3_eval::<F>(6, u, ider)
        + cc_7 * bspline_k3_eval::<F>(7, u, ider)
        + cc_8 * bspline_k3_eval::<F>(8, u, ider)
        + cc_9 * bspline_k3_eval::<F>(9, u, ider)
}
