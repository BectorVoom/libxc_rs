//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1122/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1122(t4113: f64, t840: f64, t13988: f64, t2409: f64, t4097: f64, t8734: f64, t4099: f64, t9270: f64, t4088: f64, t6781: f64, t13973: f64, t13977: f64, t13985: f64, t14002: f64, t14114: f64, t14119: f64, t14123: f64, t14128: f64, t14130: f64, t14133: f64, t14139: f64, t14141: f64, t2408: f64, t3066: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14333 = t840 * t4113;
    let t14338 = 35.0_f64 / 216.0_f64 * t13988;
    let t14340 = t2409 * t8734 * t4097;
    let t14345 = t9270 * t4099;
    let t14351 = t2409 * t6781 * t4088;
    let t14358 = 7.0_f64 / 144.0_f64 * t14333 + 7.0_f64 / 1152.0_f64 * t13973 - t13977 / 48.0_f64 - t13985 / 24.0_f64 + t14338 + t3066 * t14340 / 24.0_f64 + 7.0_f64 / 36.0_f64 * t14002 + 7.0_f64 / 288.0_f64 * t14114 - 7.0_f64 / 72.0_f64 * t14345 + t14119 / 768.0_f64 + t14123 / 8.0_f64 - 7.0_f64 / 72.0_f64 * t14128 + t2408 * t14351 / 24.0_f64 - 7.0_f64 / 72.0_f64 * t14130 - t14133 / 768.0_f64 - t14139 / 48.0_f64 + t14141 / 48.0_f64;
    (t14333, t14338, t14340, t14345, t14351, t14358)
}
