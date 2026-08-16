//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 776/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk776(t3201: f64, t3214: f64, t3221: f64, t3224: f64, t5136: f64, t5142: f64, t5143: f64, t5144: f64, t5145: f64, t5146: f64, t5147: f64, t5148: f64, t5149: f64, t5150: f64, t5151: f64) -> (f64, f64, f64, f64, f64) {
    let t5152 = 4.0_f64 / 135.0_f64 * t3201;
    let t5153 = 4.0_f64 / 405.0_f64 * t3214;
    let t5154 = 4.0_f64 / 135.0_f64 * t3221;
    let t5155 = 4.0_f64 / 405.0_f64 * t3224;
    let t5156 = -t5136 - t5142 + t5143 + t5144 + t5145 + t5146 + t5147 - t5148 + t5149 + t5150 + t5151 - t5152 - t5153 + t5154 - t5155;
    (t5152, t5153, t5154, t5155, t5156)
}
