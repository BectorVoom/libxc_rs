//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1122/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1122(t1366: f64, t5652: f64, t5655: f64, t2349: f64, t3309: f64, t3333: f64, t5649: f64, t1377: f64, t2342: f64, t97: f64, t2345: f64, t27: f64, t545: f64, t5635: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t14310 = t5652 * t1366;
    let t14312 = t5655 * t1366;
    let t14314 = t2349 * t3309;
    let t14316 = t5649 * t3333;
    let t14347 = t2342 * t97 * t1377;
    let t14350 = t2345 * t97 * t1377;
    let t14353 = t5635 * t27 * t545;
    (t14310, t14312, t14314, t14316, t14347, t14350, t14353)
}
