//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1022/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1022(t1512: f64, t1548: f64, t2857: f64, t432: f64, t1441: f64, t3213: f64, t1431: f64, t1179: f64, t161: f64, t165: f64, t177: f64, t1462: f64, t1600: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10087 = t1512 * t1548;
    let t10089 = t432 * t2857;
    let t10099 = t3213 * t1441;
    let t10109 = t3213 * t1431;
    let t10134 = 28.0_f64 / 1215.0_f64 * t161 * t1179 * t165 * t177;
    let t10139 = t1462 * t1600;
    (t10087, t10089, t10099, t10109, t10134, t10139)
}
