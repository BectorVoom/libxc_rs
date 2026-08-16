//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 770/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk770(t1302: f64, t2120: f64, t3455: f64, t786: f64, t4966: f64, t4968: f64, t4970: f64, t4972: f64, t5033: f64, t5035: f64, t5037: f64, t5039: f64, t5043: f64, t5047: f64, t5049: f64, t5051: f64, t5053: f64, t5055: f64, t5057: f64) -> (f64, f64, f64) {
    let t5059 = 4.0_f64 / 15.0_f64 * t2120 * t1302;
    let t5061 = 4.0_f64 / 15.0_f64 * t3455 * t786;
    let t5062 = -t4966 - t4968 - t4970 - t4972 - t5033 - t5035 - t5037 + t5039 + t5043 + t5047 - t5049 + t5051 - t5053 - t5055 - t5057 + t5059 + t5061;
    (t5059, t5061, t5062)
}
