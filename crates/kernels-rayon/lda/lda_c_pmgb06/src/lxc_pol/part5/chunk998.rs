//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 998/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk998(t1193: f64, t1354: f64, t18057: f64, t6716: f64, t81: f64, t118: f64, t415: f64, t6946: f64, t6928: f64, t1347: f64, t2454: f64, t117: f64, t123: f64, t315: f64, t7228: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t18059 = t18057 * t1193 * t1354;
    let t18061 = t81 * t6716;
    let t18062 = t18061 * t118;
    let t18064 = t6946 * t415;
    let t18069 = t6928 * t415;
    let t18071 = t2454 * t1347;
    let t18076 = t123 * t315 * t7228 * t117;
    (t18059, t18061, t18062, t18064, t18069, t18071, t18076)
}
