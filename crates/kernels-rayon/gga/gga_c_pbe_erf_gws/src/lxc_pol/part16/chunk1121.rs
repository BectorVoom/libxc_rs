//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1121/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1121(t13904: f64, t13907: f64, t13921: f64, t13945: f64, t13950: f64, t13954: f64, t13958: f64, t13964: f64, t13966: f64, t13969: f64, t14302: f64, t14305: f64, t14311: f64, t14322: f64, t14327: f64, t2388: f64, t2392: f64, t2408: f64, t4083: f64, t827: f64) -> f64 {
    let t14332 = t13904 / 768.0_f64 - t14302 + t13907 / 768.0_f64 - t13921 / 384.0_f64 - 7.0_f64 / 72.0_f64 * t14305 - t2392 * t4083 / 96.0_f64 - t827 * t14311 / 48.0_f64 - t2388 * t4083 / 96.0_f64 + t13945 / 48.0_f64 - t13950 / 12.0_f64 + 7.0_f64 / 72.0_f64 * t13954 - t13958 / 384.0_f64 + 7.0_f64 / 1152.0_f64 * t13964 - t2408 * t14322 / 12.0_f64 - t827 * t14327 / 48.0_f64 + t13966 / 12.0_f64 - t13969 / 24.0_f64;
    t14332
}
