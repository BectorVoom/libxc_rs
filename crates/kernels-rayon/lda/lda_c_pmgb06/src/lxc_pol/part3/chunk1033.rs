//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1033/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1033(t4803: f64, t486: f64, t490: f64, t5432: f64, t1504: f64, t1848: f64, t3073: f64, t831: f64, t132: f64, t435: f64, t4681: f64, t1842: f64, t642: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12273 = t486 * t4803 / 5.0_f64;
    let t12274 = t5432 * t490;
    let t12275 = t12274 / 15.0_f64;
    let t12276 = t1848 * t1504;
    let t12277 = 2.0_f64 / 15.0_f64 * t12276;
    let t12278 = t831 * t3073;
    let t12279 = t12278 / 15.0_f64;
    let t12281 = t132 * t435 * t4681;
    let t12282 = t12281 / 15.0_f64;
    let t12294 = 48.0_f64 * t1842 * t642;
    (t12273, t12275, t12277, t12279, t12282, t12294)
}
