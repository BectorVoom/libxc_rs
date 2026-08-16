//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1336/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1336(t13953: f64, t14781: f64, t14001: f64, t3062: f64, t14772: f64, t13888: f64, t14792: f64, t2408: f64, t29751: f64, t3066: f64, t51928: f64, t51930: f64, t54496: f64, t54502: f64, t54505: f64, t54508: f64, t54512: f64, t54519: f64, t54523: f64, t54529: f64, t9283: f64, t9702: f64) -> f64 {
    let t54531 = t13953 * t14781;
    let t54532 = 7.0_f64 / 144.0_f64 * t54531;
    let t54535 = t14001 * t3062;
    let t54536 = 7.0_f64 / 72.0_f64 * t54535;
    let t54537 = t14001 * t14772;
    let t54538 = 7.0_f64 / 72.0_f64 * t54537;
    let t54539 = -t54496 / 24.0_f64 - t54502 / 768.0_f64 + t54505 + t54508 / 384.0_f64 + t54512 / 768.0_f64 - t3066 * t29751 * t14792 / 8.0_f64 + t54519 / 48.0_f64 + t54523 / 96.0_f64 - t2408 * t9283 * t13888 * t9702 / 12.0_f64 + t54529 / 48.0_f64 + t54532 + 7.0_f64 / 4608.0_f64 * t51928 + 7.0_f64 / 288.0_f64 * t51930 - t54536 + t54538;
    t54539
}
