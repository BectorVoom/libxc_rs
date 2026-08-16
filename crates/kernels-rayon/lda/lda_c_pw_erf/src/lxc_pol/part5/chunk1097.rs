//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1097/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1097(t127: f64, t14797: f64, t14803: f64, t14814: f64, t14817: f64, t20396: f64, t20397: f64, t20403: f64, t20406: f64, t20409: f64, t20412: f64, t411: f64, t7918: f64, t9037: f64) -> f64 {
    let t20417 = 5.87616_f64 * t14797 + t14803 + t14814 + t14817 + t20396 + 5.87616_f64 * t127 * t20397 * t411 + t20403 - t20406 + t20409 + t20412 + 176.2848_f64 * t127 * t9037 * t7918 * t411;
    t20417
}
