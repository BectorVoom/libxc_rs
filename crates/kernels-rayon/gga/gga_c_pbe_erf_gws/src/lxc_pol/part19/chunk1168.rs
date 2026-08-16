//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1168/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1168(t14026: f64, t14481: f64, t14483: f64, t14485: f64, t14487: f64, t14489: f64, t14491: f64, t14493: f64, t14495: f64, t14499: f64, t14502: f64, t14506: f64) -> (f64, f64) {
    let t15049 = -t14481 / 192.0_f64 + t14483 / 48.0_f64 - t14485 / 384.0_f64 + t14487 / 96.0_f64 - t14489 / 384.0_f64 + t14491 / 48.0_f64 - t14493 / 192.0_f64 - t14495 / 48.0_f64 + t14499 / 128.0_f64 - t14026 + t14502 / 384.0_f64;
    let t15050 = 7.0_f64 / 576.0_f64 * t14506;
    (t15049, t15050)
}
