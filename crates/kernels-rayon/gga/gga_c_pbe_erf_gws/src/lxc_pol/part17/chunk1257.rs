//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1257/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1257(t13772: f64, t13930: f64, t13939: f64, t22379: f64, t4385: f64, t50977: f64, t53432: f64, t53435: f64, t53439: f64, t53444: f64, t53449: f64, t53460: f64, t53464: f64, t53468: f64, t53472: f64, t53476: f64, t53481: f64, t8629: f64, t8654: f64) -> f64 {
    let t53483 = -t53432 / 1536.0_f64 - t53435 / 384.0_f64 - t53439 / 768.0_f64 + t53444 / 192.0_f64 + t53449 / 768.0_f64 - t8629 * t50977 / 24.0_f64 - t8654 * t13939 / 48.0_f64 - t8654 * t13772 / 48.0_f64 + t22379 * t13930 / 24.0_f64 - t53460 / 1536.0_f64 + t4385 * t53464 / 96.0_f64 + t53468 / 1536.0_f64 - t4385 * t53472 / 48.0_f64 + t53476 / 192.0_f64 + t53481 / 384.0_f64;
    t53483
}
