//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1036/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1036(t108: f64, t2119: f64, t267: f64, t3970: f64, t3482: f64, t3965: f64, t6766: f64, t2103: f64, t4476: f64, t9513: f64, t4804: f64, t4826: f64) -> (f64, f64, f64, f64, f64) {
    let t12136 = t2119 * t108 * t267;
    let t12138 = 16.0_f64 / 15.0_f64 * t12136 * t3970;
    let t12141 = 8.0_f64 / 9.0_f64 * t3965 * t6766 * t3482;
    let t12143 = t2103 * t108 * t267;
    let t12145 = 16.0_f64 / 15.0_f64 * t12143 * t4476;
    let t12146 = 16.0_f64 / 45.0_f64 * t9513;
    let t12148 = 8.0_f64 / 15.0_f64 * t4804 * t4826;
    (t12138, t12141, t12145, t12146, t12148)
}
