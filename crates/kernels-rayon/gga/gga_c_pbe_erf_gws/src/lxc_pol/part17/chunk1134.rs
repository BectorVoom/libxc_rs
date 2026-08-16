//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1134/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1134(t14498: f64, t3249: f64, t3299: f64, t4039: f64, t14025: f64, t14481: f64, t14483: f64, t14485: f64, t14487: f64, t14489: f64, t14491: f64, t14493: f64, t14495: f64) -> f64 {
    let t14499 = t14498 * t3249;
    let t14502 = t4039 * t3299;
    let t14504 = -t14481 / 384.0_f64 + t14483 / 96.0_f64 - t14485 / 768.0_f64 + t14487 / 192.0_f64 - t14489 / 768.0_f64 + t14491 / 96.0_f64 - t14493 / 384.0_f64 - t14495 / 96.0_f64 + t14499 / 256.0_f64 - 7.0_f64 / 288.0_f64 * t14025 + t14502 / 768.0_f64;
    t14504
}
