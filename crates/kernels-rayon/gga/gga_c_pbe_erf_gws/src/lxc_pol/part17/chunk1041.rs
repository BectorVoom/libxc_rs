//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1041/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1041(t3219: f64, t3235: f64, t8904: f64, t2277: f64, t8894: f64, t8899: f64, t8901: f64, t8908: f64, t8912: f64, t8917: f64, t8923: f64, t902: f64, t9411: f64, t9415: f64, t9417: f64, t9421: f64, t9425: f64) -> (f64, f64) {
    let t9427 = t3235 * t3219 * t8904;
    let t9430 = -t8894 + t902 * t9411 / 1536.0_f64 + t8899 - t9415 - t8901 + t902 * t9417 / 768.0_f64 - t8908 + t8912 - t2277 * t9421 / 1536.0_f64 + t8917 - t9425 * t9427 / 128.0_f64 + t8923;
    (t9427, t9430)
}
