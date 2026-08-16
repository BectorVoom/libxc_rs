//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1221/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1221(t11952: f64, t132: f64, t137: f64, t2648: f64, t3058: f64, t11971: f64, t12022: f64, t12037: f64, t12039: f64, t12036: f64, t835: f64, t2462: f64, t3223: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t16095 = 8.0_f64 / 135.0_f64 * t11952;
    let t16099 = t132 * t137 * t3058 * t2648 / 30.0_f64;
    let t16100 = 4.0_f64 / 135.0_f64 * t11971;
    let t16101 = 8.0_f64 / 135.0_f64 * t12022;
    let t16102 = 8.0_f64 / 405.0_f64 * t12037;
    let t16103 = 8.0_f64 / 135.0_f64 * t12039;
    let t16104 = t12036 * t835;
    let t16105 = 4.0_f64 / 405.0_f64 * t16104;
    let t16106 = t3223 * t2462;
    (t16095, t16099, t16100, t16101, t16102, t16103, t16105, t16106)
}
