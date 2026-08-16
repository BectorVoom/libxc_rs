//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1120/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1120(t3155: f64, t831: f64, t1395: f64, t1531: f64, t5077: f64, t5086: f64, t177: f64, t2911: f64, t12529: f64, t12547: f64, t2918: f64, t5138: f64) -> (f64, f64, f64, f64) {
    let t13294 = t831 * t3155;
    let t13295 = t13294 / 45.0_f64;
    let t13296 = t1395 * t1531;
    let t13299 = 4.0_f64 / 15.0_f64 * t5077 * t13296 * t5086;
    let t13300 = t177 * t2911;
    let t13303 = 8.0_f64 / 27.0_f64 * t12529 * t13300 * t12547;
    let t13304 = t177 * t2918;
    let t13307 = 2.0_f64 / 3.0_f64 * t5138 * t13304 * t12547;
    (t13295, t13299, t13303, t13307)
}
