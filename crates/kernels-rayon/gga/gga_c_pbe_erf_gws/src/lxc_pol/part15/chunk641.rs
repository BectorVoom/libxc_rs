//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 641/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk641(t3083: f64, t833: f64, t1164: f64, t840: f64, t1115: f64, t2225: f64, t2236: f64, t2247: f64, t2362: f64, t2373: f64, t2408: f64, t2498: f64, t2503: f64, t3040: f64, t3047: f64, t3052: f64, t3055: f64, t3062: f64, t3066: f64, t3070: f64, t3077: f64, t3079: f64, t827: f64) -> f64 {
    let t3084 = t3083 * t833;
    let t3086 = t840 * t1164;
    let t3088 = t2498 * t833 / 96.0_f64 + t827 * t2503 / 96.0_f64 + t3040 * t833 / 96.0_f64 + 7.0_f64 / 288.0_f64 * t2236 + 7.0_f64 / 144.0_f64 * t2247 - t827 * t3047 / 96.0_f64 - t827 * t3052 / 48.0_f64 - t3055 * t2362 / 96.0_f64 - t1115 * t2373 / 48.0_f64 + t2408 * t3062 / 48.0_f64 + t3066 * t3070 / 48.0_f64 + t3077 * t3079 / 96.0_f64 - 7.0_f64 / 288.0_f64 * t2225 - 7.0_f64 / 288.0_f64 * t3084 + 7.0_f64 / 288.0_f64 * t3086;
    t3088
}
