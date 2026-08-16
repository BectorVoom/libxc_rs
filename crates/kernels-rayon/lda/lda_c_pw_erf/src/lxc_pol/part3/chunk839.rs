//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 839/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk839(t1901: f64, t646: f64, t3985: f64, t3988: f64, t3992: f64, t4955: f64, t4961: f64, t4963: f64, t4966: f64, t4968: f64, t4970: f64, t4972: f64, t5033: f64, t5035: f64, t5037: f64, t5039: f64, t5043: f64, t5047: f64) -> f64 {
    let t5859 = t1901 * t646;
    let t5861 = t4955 - t4961 - t4963 - t4966 - t4968 - t4970 - 4.0_f64 / 27.0_f64 * t3985 - t3988 + t3992 - t4972 - t5033 - t5035 - t5037 + 0.033245444444444446_f64 * t5859 + t5039 + t5043 + t5047;
    t5861
}
