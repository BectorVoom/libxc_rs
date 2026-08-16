//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 779/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk779(t211: f64, t5170: f64, t1405: f64, t822: f64, t2071: f64, t4567: f64, t548: f64, t1397: f64, t2076: f64, t5067: f64, t5071: f64, t5131: f64, t5133: f64, t5135: f64, t5140: f64, t5145: f64, t5150: f64, t5154: f64, t5159: f64, t5164: f64, t5169: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5172 = 8.0_f64 / 45.0_f64 * t211 * t5170;
    let t5174 = 4.0_f64 / 15.0_f64 * t822 * t1405;
    let t5175 = t4567 * t2071;
    let t5176 = t548 * t5175;
    let t5177 = 4.0_f64 / 9.0_f64 * t5176;
    let t5179 = 16.0_f64 / 45.0_f64 * t2076 * t1397;
    let t5180 = t5067 + t5071 - t5131 - t5133 + t5135 - t5140 - t5145 + t5150 - t5154 - t5159 - t5164 + t5169 - t5172 + t5174 + t5177 + t5179;
    (t5172, t5174, t5175, t5177, t5179, t5180)
}
