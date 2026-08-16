//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1320/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1320(t27047: f64, t4216: f64, t9296: f64, t938: f64, t52154: f64, t52294: f64, t52393: f64, t52534: f64, t53426: f64, t53432: f64, t53435: f64, t53439: f64, t53444: f64, t53449: f64, t53460: f64, t53468: f64, t53476: f64, t53481: f64, t6793: f64, t8629: f64, t8793: f64) -> f64 {
    let t55182 = t27047 * t9296 * t4216 * t938;
    let t55187 = t53426 / 24.0_f64 - t53432 / 768.0_f64 - t53435 / 192.0_f64 - t53439 / 384.0_f64 - t8793 * t52154 / 12.0_f64 + t53444 / 96.0_f64 + t53449 / 384.0_f64 - t53460 / 768.0_f64 + t8629 * t52534 / 48.0_f64 + t8793 * t52393 / 24.0_f64 - t8629 * t52294 / 24.0_f64 + t53468 / 768.0_f64 - t6793 * t55182 / 8.0_f64 + t53476 / 96.0_f64 + t53481 / 192.0_f64;
    t55187
}
