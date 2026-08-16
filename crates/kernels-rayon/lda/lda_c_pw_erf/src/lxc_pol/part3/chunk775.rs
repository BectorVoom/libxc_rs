//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 775/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk775(t5094: f64, t5126: f64, t530: f64, t186: f64, t185: f64, t1383: f64, t822: f64, t1289: f64, t2076: f64, t494: f64, t739: f64, t3967: f64, t542: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5127 = t5094 + t5126;
    let t5128 = t530 * t5127;
    let t5129 = t186 * t5128;
    let t5131 = 2.0_f64 / 15.0_f64 * t185 * t5129;
    let t5133 = 2.0_f64 / 15.0_f64 * t822 * t1383;
    let t5135 = 4.0_f64 / 15.0_f64 * t2076 * t1289;
    let t5136 = t739 * t494;
    let t5138 = t3967 * t5136 * t542;
    (t5127, t5128, t5129, t5131, t5133, t5135, t5136, t5138)
}
