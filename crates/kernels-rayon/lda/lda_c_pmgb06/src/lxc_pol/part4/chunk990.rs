//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 990/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk990(t1018: f64, t138: f64, t3698: f64, t1023: f64, t1026: f64, t409: f64, t109: f64, t3674: f64, t3676: f64, t1767: f64, t282: f64, t55: f64, t691: f64) -> (f64, f64, f64, f64) {
    let t8647 = 0.07123333333333333_f64 * t138 * t1018 * t3698;
    let t8651 = 0.2849333333333333_f64 * t138 * t409 * t1023 * t1026;
    let t8655 = 6.87343803774119_f64 * t138 * t109 * t3674 * t3676;
    let t8659 = 0.0018989649058080863_f64 * t691 * t55 * t1767 * t282;
    (t8647, t8651, t8655, t8659)
}
