//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 781/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk781(t1298: f64, t2131: f64, t2134: f64, t511: f64, t2114: f64, t2127: f64, t4060: f64, t4064: f64, t4041: f64, t4215: f64, t4217: f64, t5181: f64, t5182: f64, t5183: f64, t5186: f64, t5188: f64, t5190: f64, t5192: f64, t5194: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5196 = 8.0_f64 / 15.0_f64 * t1298 * t2131;
    let t5198 = 8.0_f64 / 45.0_f64 * t511 * t2134;
    let t5200 = 16.0_f64 / 45.0_f64 * t2114 * t2127;
    let t5202 = 8.0_f64 / 15.0_f64 * t2114 * t2131;
    let t5203 = 8.0_f64 / 135.0_f64 * t4060;
    let t5204 = 8.0_f64 / 81.0_f64 * t4064;
    let t5205 = -t5181 + t5182 + t5183 + t4041 - t5186 + t5188 + t5190 + t5192 + t5194 + t5196 - t5198 + t5200 + t5202 + t5203 + t5204 + t4215 + t4217;
    (t5196, t5198, t5200, t5202, t5203, t5204, t5205)
}
