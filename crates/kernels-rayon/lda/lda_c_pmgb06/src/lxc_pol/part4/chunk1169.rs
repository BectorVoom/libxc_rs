//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1169/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1169(t1074: f64, t6145: f64, t1525: f64, t36: f64, t1069: f64, t2377: f64, t9220: f64, t3090: f64, t6150: f64, t9190: f64, t9188: f64, t350: f64, t6186: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15373 = t6145 * t1074;
    let t15375 = t36 * t1525 * t15373;
    let t15378 = t9220 * t2377 * t1069;
    let t15380 = t36 * t3090 * t15378;
    let t15382 = t6150 * t1074;
    let t15384 = t36 * t3090 * t15382;
    let t15387 = t9190 * t2377 * t1069;
    let t15389 = t36 * t9188 * t15387;
    let t15391 = t350 * t6186;
    (t15373, t15375, t15378, t15380, t15382, t15384, t15387, t15389, t15391)
}
