//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 828/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk828(t7468: f64, t7545: f64, t7631: f64, t7679: f64, t7709: f64, t7745: f64, t7764: f64, t7866: f64, t3: f64, t7364: f64, t117: f64, t118: f64, t123: f64, t125: f64, t2793: f64, t2797: f64, t2812: f64, t2820: f64, t2825: f64, t2840: f64, t2844: f64, t2846: f64, t3481: f64, t7157: f64) -> (f64, f64, f64) {
    let t7869 = t7468 + t7545 + t7631 + t7679 + t7709 + t7745 + t7764 + t7866;
    let t7874 = t3 * t7364;
    let t7878 = -t2793 + t2797 - t2812 + t2820 + t2825 - t2840 - t2844 - t2846 + t3481 - 0.005388405304614574_f64 * t123 * t125 * t7869 * t117 - 0.031505407223141116_f64 * t7874 * t118 - 0.005926167098672845_f64 * t7157;
    (t7869, t7874, t7878)
}
