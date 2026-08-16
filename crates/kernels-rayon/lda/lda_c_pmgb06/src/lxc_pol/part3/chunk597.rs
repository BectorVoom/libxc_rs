//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 597/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk597(t170: f64, t3247: f64, t176: f64, t2911: f64, t2912: f64, t493: f64, t3115: f64, t444: f64, t442: f64, t439: f64, t135: f64, t1531: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3248 = t3247 * t170;
    let t3249 = t176 * t2911;
    let t3250 = t3249 * t2912;
    let t3251 = t3248 * t3250;
    let t3253 = 8.0_f64 / 81.0_f64 * t493 * t3251;
    let t3254 = t444 * t3115;
    let t3255 = t442 * t3254;
    let t3257 = t439 * t3255 / 45.0_f64;
    let t3259 = 1.0_f64 / t135 / t1531;
    (t3248, t3250, t3251, t3253, t3254, t3255, t3257, t3259)
}
