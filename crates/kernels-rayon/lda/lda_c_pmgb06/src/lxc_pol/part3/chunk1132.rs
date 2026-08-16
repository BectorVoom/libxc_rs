//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1132/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1132(t10190: f64, t10196: f64, t1966: f64, t1967: f64, t3441: f64, t439: f64, t1972: f64, t3251: f64, t835: f64, t9370: f64, t1977: f64, t3198: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13456 = 2.0_f64 / 45.0_f64 * t10190;
    let t13457 = 2.0_f64 / 27.0_f64 * t10196;
    let t13461 = t439 * t1966 * t1967 * t3441 / 15.0_f64;
    let t13463 = 8.0_f64 / 81.0_f64 * t1972 * t3251;
    let t13465 = t9370 * t835 / 45.0_f64;
    let t13467 = t3198 * t1977 / 15.0_f64;
    (t13456, t13457, t13461, t13463, t13465, t13467)
}
