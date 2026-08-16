//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1243/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1243(t13984: f64, t53229: f64, t1192: f64, t13835: f64, t14420: f64, t14622: f64, t19704: f64, t20113: f64, t2376: f64, t2408: f64, t2409: f64, t3066: f64, t51030: f64, t53187: f64, t53189: f64, t53199: f64, t53207: f64, t53212: f64, t53220: f64, t53224: f64, t53227: f64, t6793: f64, t8574: f64, t8589: f64, t8734: f64) -> f64 {
    let t53230 = t53229 * t13984;
    let t53231 = 7.0_f64 / 144.0_f64 * t53230;
    let t53232 = -t53187 - t53189 + t2408 * t2409 * t8589 * t13835 / 24.0_f64 + t3066 * t2409 * t8734 * t14622 / 24.0_f64 - t53199 + t2408 * t2409 * t2376 * t1192 * t8574 / 48.0_f64 - 5.0_f64 / 768.0_f64 * t53207 + 7.0_f64 / 48.0_f64 * t51030 + t53212 / 384.0_f64 + t19704 * t14420 / 48.0_f64 + t20113 * t14420 / 48.0_f64 + t6793 * t53220 / 24.0_f64 - t53224 + t53227 / 768.0_f64 + t53231;
    t53232
}
