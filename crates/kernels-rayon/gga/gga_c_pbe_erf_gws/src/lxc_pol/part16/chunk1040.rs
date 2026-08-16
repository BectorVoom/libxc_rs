//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1040/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1040(t2253: f64, t2277: f64, t2343: f64, t3247: f64, t6637: f64, t8878: f64, t8883: f64, t8889: f64, t9366: f64, t9372: f64, t9377: f64, t9382: f64, t9389: f64, t9393: f64, t9397: f64, t9401: f64, t9406: f64) -> f64 {
    let t9409 = t2277 * t9366 / 384.0_f64 - t8878 - t2253 * t9372 / 384.0_f64 - t2253 * t9377 / 384.0_f64 - t2253 * t9382 / 768.0_f64 - t6637 * t9389 / 192.0_f64 - t8883 - t2343 * t9393 / 1536.0_f64 + t2343 * t9397 / 384.0_f64 + t3247 * t9401 / 256.0_f64 + t2343 * t9406 / 192.0_f64 - t8889;
    t9409
}
