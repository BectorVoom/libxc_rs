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
//! (`idx`/`i`/`ider`) alongside their generic `` float parameters.
//! The float portion of each signature is fully generic.


/// Get knot value by index (inlined constant lookup for CubeCL).
/// Case21: k=3, Nsp=10, knots[i] = (-3 + i) / 7
fn knot(idx: u32) -> f64 {
    let i = (idx as f64);
    (i - 3.0_f64) / 7.0_f64
}

/// Safe division: returns 0 if denominator is 0, otherwise a/b.
fn safe_div(a: f64, b: f64) -> f64 {
    (if b == 0.0_f64 { 0.0_f64 } else { a / b })
}

/// Evaluate a single B-spline basis function N_{i,3}(u) for derivative order `ider`.
///
/// For k=3, supports ider=0..3 (ider>=4 returns 0 since maxk=min(4,3)=3).
/// Uses `if/else` guards matching libxc's `xc_bspline` control flow: skips
/// computation outside support, dispatches only the needed derivative order.
fn bspline_k3_eval(i: u32, u: f64, ider: u32) -> f64 {
    let ki0 = knot(i);
    let ki1 = knot(i + 1);
    let ki2 = knot(i + 2);
    let ki3 = knot(i + 3);
    let ki4 = knot(i + 4);

    let mut result = 0.0_f64;

    // Guard: only compute if u is in support [knots[i], knots[i+4])
    if u >= ki0 {
        if u < ki4 {
            // Degree 0: piecewise constants (always needed)
            let n0_0 = (if u >= ki0 { (if u < ki1 { 1.0_f64 } else { 0.0_f64 }) } else { 0.0_f64 });
            let n0_1 = (if u >= ki1 { (if u < ki2 { 1.0_f64 } else { 0.0_f64 }) } else { 0.0_f64 });
            let n0_2 = (if u >= ki2 { (if u < ki3 { 1.0_f64 } else { 0.0_f64 }) } else { 0.0_f64 });
            let n0_3 = (if u >= ki3 { (if u < ki4 { 1.0_f64 } else { 0.0_f64 }) } else { 0.0_f64 });

            if ider == 3 {
                // Derivative order 3: only needs N[0] values, 3 triangular passes
                let d3_a0 = safe_div(n0_0, ki1 - ki0);
                let d3_a1 = safe_div(n0_1, ki2 - ki1);
                let d3_a2 = safe_div(n0_2, ki3 - ki2);
                let d3_a3 = safe_div(n0_3, ki4 - ki3);
                let d3_b0 = d3_a0 - d3_a1;
                let d3_b1 = d3_a1 - d3_a2;
                let d3_b2 = d3_a2 - d3_a3;
                let d3_c0 = safe_div(d3_b0, ki2 - ki0);
                let d3_c1 = safe_div(d3_b1, ki3 - ki1);
                let d3_c2 = safe_div(d3_b2, ki4 - ki2);
                let d3_d0 = 2.0_f64 * (d3_c0 - d3_c1);
                let d3_d1 = 2.0_f64 * (d3_c1 - d3_c2);
                let d3_e0 = safe_div(d3_d0, ki3 - ki0);
                let d3_e1 = safe_div(d3_d1, ki4 - ki1);
                result = 3.0_f64 * (d3_e0 - d3_e1);
            } else {
                // Degree 1: needed for ider <= 2
                let n1_0 = safe_div((u - ki0) * n0_0, ki1 - ki0)
                         + safe_div((ki2 - u) * n0_1, ki2 - ki1);
                let n1_1 = safe_div((u - ki1) * n0_1, ki2 - ki1)
                         + safe_div((ki3 - u) * n0_2, ki3 - ki2);
                let n1_2 = safe_div((u - ki2) * n0_2, ki3 - ki2)
                         + safe_div((ki4 - u) * n0_3, ki4 - ki3);

                if ider == 2 {
                    // Derivative order 2: needs N[1] values, 2 triangular passes
                    let d2_a0 = safe_div(n1_0, ki2 - ki0);
                    let d2_a1 = safe_div(n1_1, ki3 - ki1);
                    let d2_a2 = safe_div(n1_2, ki4 - ki2);
                    let d2_b0 = 2.0_f64 * (d2_a0 - d2_a1);
                    let d2_b1 = 2.0_f64 * (d2_a1 - d2_a2);
                    let d2_c0 = safe_div(d2_b0, ki3 - ki0);
                    let d2_c1 = safe_div(d2_b1, ki4 - ki1);
                    result = 3.0_f64 * (d2_c0 - d2_c1);
                } else {
                    // Degree 2: needed for ider <= 1
                    let n2_0 = safe_div((u - ki0) * n1_0, ki2 - ki0)
                             + safe_div((ki3 - u) * n1_1, ki3 - ki1);
                    let n2_1 = safe_div((u - ki1) * n1_1, ki3 - ki1)
                             + safe_div((ki4 - u) * n1_2, ki4 - ki2);

                    if ider == 1 {
                        // Derivative order 1: needs N[2] values
                        let d1_s0 = safe_div(n2_0, ki3 - ki0);
                        let d1_s1 = safe_div(n2_1, ki4 - ki1);
                        result = 3.0_f64 * (d1_s0 - d1_s1);
                    } else {
                        // ider == 0: function value, needs full N[3]
                        result = safe_div((u - ki0) * n2_0, ki3 - ki0)
                               + safe_div((ki4 - u) * n2_1, ki4 - ki1);
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
#[allow(clippy::too_many_arguments)]
pub fn case21_xbspline(
    u: f64, ider: u32,
    cx_0: f64, cx_1: f64, cx_2: f64, cx_3: f64, cx_4: f64,
    cx_5: f64, cx_6: f64, cx_7: f64, cx_8: f64, cx_9: f64,
) -> f64 {
    cx_0 * bspline_k3_eval(0, u, ider)
        + cx_1 * bspline_k3_eval(1, u, ider)
        + cx_2 * bspline_k3_eval(2, u, ider)
        + cx_3 * bspline_k3_eval(3, u, ider)
        + cx_4 * bspline_k3_eval(4, u, ider)
        + cx_5 * bspline_k3_eval(5, u, ider)
        + cx_6 * bspline_k3_eval(6, u, ider)
        + cx_7 * bspline_k3_eval(7, u, ider)
        + cx_8 * bspline_k3_eval(8, u, ider)
        + cx_9 * bspline_k3_eval(9, u, ider)
}

/// Evaluate case21 correlation B-spline: sum_i cc[i] * B_{i,3}(u, ider)
///
/// cc_0..cc_9 are the 10 correlation enhancement coefficients.
#[allow(clippy::too_many_arguments)]
pub fn case21_cbspline(
    u: f64, ider: u32,
    cc_0: f64, cc_1: f64, cc_2: f64, cc_3: f64, cc_4: f64,
    cc_5: f64, cc_6: f64, cc_7: f64, cc_8: f64, cc_9: f64,
) -> f64 {
    cc_0 * bspline_k3_eval(0, u, ider)
        + cc_1 * bspline_k3_eval(1, u, ider)
        + cc_2 * bspline_k3_eval(2, u, ider)
        + cc_3 * bspline_k3_eval(3, u, ider)
        + cc_4 * bspline_k3_eval(4, u, ider)
        + cc_5 * bspline_k3_eval(5, u, ider)
        + cc_6 * bspline_k3_eval(6, u, ider)
        + cc_7 * bspline_k3_eval(7, u, ider)
        + cc_8 * bspline_k3_eval(8, u, ider)
        + cc_9 * bspline_k3_eval(9, u, ider)
}
