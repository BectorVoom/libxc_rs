//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 384/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk384(t1427: f64, t439: f64, t1074: f64, t444: f64, t442: f64) -> (f64, f64, f64) {
    let t1429 = 2.0_f64 / 45.0_f64 * t439 * t1427;
    let t1430 = t444 * t1074;
    let t1431 = t442 * t1430;
    (t1429, t1430, t1431)
}
