//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 901/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk901(t3421: f64, t405: f64, t9177: f64, t3424: f64, t3418: f64, t161: f64, t3039: f64, t489: f64, t1697: f64, t1730: f64, t3073: f64, t486: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9719 = t405 * t3421;
    let t9724 = 0.3732469135802469_f64 * t9177;
    let t9737 = t405 * t3424;
    let t9739 = t405 * t3418;
    let t9754 = t161 * t489 * t3039;
    let t9759 = 0.19947266666666666_f64 * t1697 * t1730;
    let t9760 = t486 * t3073;
    (t9719, t9724, t9737, t9739, t9754, t9759, t9760)
}
