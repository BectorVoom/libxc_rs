//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1009/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1009(t6961: f64, t707: f64, t6957: f64, t2448: f64, t384: f64, t1147: f64, t123: f64, t2407: f64, t317: f64, t113: f64, t2414: f64, t247: f64, t301: f64) -> (f64, f64, f64, f64, f64) {
    let t18883 = t707 * t6961;
    let t18885 = t707 * t6957;
    let t18903 = t384 * t2448;
    let t18911 = t123 * t1147 * t2407 * t317;
    let t18915 = t247 * t2414 * t113 * t301;
    (t18883, t18885, t18903, t18911, t18915)
}
