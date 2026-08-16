//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 926/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk926(t1126: f64, t1147: f64, t123: f64, t317: f64, t4001: f64, t701: f64, t117: f64, t550: f64, t1366: f64, t3312: f64, t3319: f64, t3333: f64) -> (f64, f64, f64, f64, f64) {
    let t10657 = t123 * t1147 * t1126 * t317;
    let t10661 = t123 * t4001 * t701 * t317;
    let t10670 = t123 * t1147 * t550 * t117;
    let t10679 = t3312 * t1366;
    let t10681 = t3319 * t3333;
    (t10657, t10661, t10670, t10679, t10681)
}
