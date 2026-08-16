//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1011/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1011(t3005: f64, t486: f64, t1455: f64, t3223: f64, t1467: f64, t1499: f64, t1555: f64, t3155: f64, t1767: f64, t206: f64, t4068: f64, t4077: f64, t591: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9352 = t486 * t3005;
    let t9379 = t3223 * t1455;
    let t9381 = t3223 * t1467;
    let t9402 = t1499 * t1555;
    let t9404 = t486 * t3155;
    let t9408 = 0.008082336938271605_f64 * t206 * t1767 * t4068;
    let t9410 = 8.0_f64 / 9.0_f64 * t4077 * t591;
    (t9352, t9379, t9381, t9402, t9404, t9408, t9410)
}
