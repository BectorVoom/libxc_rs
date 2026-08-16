//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 892/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk892(t3148: f64, t335: f64, t913: f64, t904: f64, t914: f64, t935: f64, t3115: f64, t3136: f64, t905: f64, t987: f64, t973: f64, t990: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8536 = 8.0_f64 * t913 * t335 * t3148;
    let t8539 = 36.0_f64 * t904 * t914 * t935;
    let t8542 = 578.9456755974397_f64 * t3136 * t3115 * t905;
    let t8561 = t987 * t987;
    let t8564 = t973 * t973;
    let t8565 = t990 * t990;
    (t8536, t8539, t8542, t8561, t8564, t8565)
}
