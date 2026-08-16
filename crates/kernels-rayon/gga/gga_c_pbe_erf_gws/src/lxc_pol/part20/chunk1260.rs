//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1260/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1260(t13953: f64, t14648: f64, t13972: f64, t14684: f64, t14473: f64, t840: f64, t14579: f64, t14576: f64, t2376: f64, t829: f64, t830: f64, t14608: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t54429 = t13953 * t14648;
    let t54430 = 7.0_f64 / 144.0_f64 * t54429;
    let t54463 = t13972 * t14684;
    let t54464 = 7.0_f64 / 1152.0_f64 * t54463;
    let t54480 = 7.0_f64 / 144.0_f64 * t840 * t14473;
    let t54482 = 7.0_f64 / 144.0_f64 * t840 * t14579;
    let t54486 = t2376 * t14576;
    let t54488 = t829 * t830 * t54486;
    let t54491 = t13972 * t14608;
    (t54430, t54464, t54480, t54482, t54488, t54491)
}
