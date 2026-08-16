//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1046/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1046(t3860: f64, t4738: f64, t10056: f64, t2967: f64, t743: f64, t4776: f64, t571: f64, t2018: f64, t3727: f64, t1472: f64, t4773: f64, t4777: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12251 = t4738 * t3860;
    let t12252 = 32.0_f64 / 45.0_f64 * t12251;
    let t12254 = t10056 * t743 * t2967;
    let t12257 = 128.0_f64 / 27.0_f64 * t571 * t4776 * t12254;
    let t12259 = 4.0_f64 / 9.0_f64 * t3727 * t2018;
    let t12261 = 4.0_f64 / 9.0_f64 * t1472 * t4773;
    let t12263 = 32.0_f64 / 27.0_f64 * t1472 * t4777;
    (t12252, t12254, t12257, t12259, t12261, t12263)
}
