//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1172/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1172(t1069: f64, t2381: f64, t3092: f64, t3090: f64, t36: f64, t2579: f64, t947: f64, t2571: f64, t1525: f64, t1830: f64, t1858: f64, t2575: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15411 = t3092 * t2381 * t1069;
    let t15413 = t36 * t3090 * t15411;
    let t15416 = t947 * t2579;
    let t15418 = t947 * t2571;
    let t15421 = t1830 * t1525 * t1858;
    let t15423 = t947 * t2575;
    (t15411, t15413, t15416, t15418, t15421, t15423)
}
