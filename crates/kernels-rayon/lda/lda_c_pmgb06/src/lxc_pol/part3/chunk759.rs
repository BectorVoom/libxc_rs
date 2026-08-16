//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 759/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk759(t5071: f64, t5139: f64, t5138: f64, t3074: f64, t3077: f64, t3149: f64, t3151: f64, t3153: f64, t3156: f64, t3158: f64, t3165: f64, t3182: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5140 = t5139 * t5071;
    let t5142 = 2.0_f64 / 27.0_f64 * t5138 * t5140;
    let t5143 = t3074 / 45.0_f64;
    let t5144 = t3077 / 45.0_f64;
    let t5145 = t3149 / 45.0_f64;
    let t5146 = 2.0_f64 / 45.0_f64 * t3151;
    let t5147 = 2.0_f64 / 45.0_f64 * t3153;
    let t5148 = 2.0_f64 / 135.0_f64 * t3156;
    let t5149 = t3158 / 45.0_f64;
    let t5150 = 4.0_f64 / 135.0_f64 * t3165;
    let t5151 = 4.0_f64 / 135.0_f64 * t3182;
    (t5140, t5142, t5143, t5144, t5145, t5146, t5147, t5148, t5149, t5150, t5151)
}
