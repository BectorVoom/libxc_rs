//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 841/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk841(t4041: f64, t5164: f64, t5169: f64, t5172: f64, t5174: f64, t5177: f64, t5179: f64, t5181: f64, t5182: f64, t5183: f64, t5186: f64, t5188: f64, t5190: f64, t5192: f64, t5194: f64, t5196: f64, t5198: f64) -> f64 {
    let t5864 = -t5164 + t5169 - t5172 + t5174 + t5177 + t5179 - t5181 + t5182 + t5183 + t4041 - t5186 + t5188 + t5190 + t5192 + t5194 + t5196 - t5198;
    t5864
}
