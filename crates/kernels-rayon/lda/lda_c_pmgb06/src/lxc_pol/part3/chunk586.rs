//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 586/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk586(t3177: f64, t446: f64, t1444: f64, t1451: f64, t1447: f64, t1420: f64, t1560: f64, t1426: f64, t1559: f64, t439: f64, t153: f64, t3098: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3179 = t3177 * t446 / 15.0_f64;
    let t3181 = 2.0_f64 / 15.0_f64 * t1444 * t1451;
    let t3182 = t1447 * t1451;
    let t3183 = 4.0_f64 / 45.0_f64 * t3182;
    let t3185 = 2.0_f64 / 15.0_f64 * t1420 * t1560;
    let t3186 = t1426 * t1559;
    let t3188 = 2.0_f64 / 15.0_f64 * t439 * t3186;
    let t3189 = t153 * t3098;
    (t3179, t3181, t3182, t3183, t3185, t3186, t3188, t3189)
}
