//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1116/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1116(t6729: f64, t941: f64, t2352: f64, t6726: f64, t840: f64, t2376: f64, t6723: f64, t829: f64, t830: f64, t19999: f64, t20007: f64, t20009: f64, t20017: f64, t20024: f64, t2220: f64, t2379: f64, t2408: f64, t2409: f64, t2416: f64, t2418: f64, t335: f64, t338: f64, t353: f64, t4379: f64, t4427: f64, t6107: f64, t6739: f64, t827: f64, t938: f64, t939: f64) -> f64 {
    let t20026 = t6729 * t941;
    let t20028 = t2352 * t2352;
    let t20034 = t840 * t6726;
    let t20036 = t2376 * t6723;
    let t20038 = t829 * t830 * t20036;
    let t20043 = 7.0_f64 / 24.0_f64 * t19999 - t6107 * t2379 / 24.0_f64 - t335 * t338 * t6739 * t939 / 24.0_f64 + 7.0_f64 / 12.0_f64 * t20007 + 35.0_f64 / 36.0_f64 * t20009 + t335 * t338 * t2220 * t2418 / 8.0_f64 + 35.0_f64 / 72.0_f64 * t20017 + t2408 * t2409 * t2376 * t4379 * t938 / 12.0_f64 + 7.0_f64 / 6.0_f64 * t20024 + 455.0_f64 / 324.0_f64 * t20026 + t335 * t338 * t353 * t2416 * t20028 / 16.0_f64 + 7.0_f64 / 72.0_f64 * t20034 - t827 * t20038 / 24.0_f64 - t4427 * t2379 / 12.0_f64;
    t20043
}
