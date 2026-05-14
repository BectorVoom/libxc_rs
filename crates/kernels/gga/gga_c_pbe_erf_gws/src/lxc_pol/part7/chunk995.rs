//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 995/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk995<F: Float>(t2376: F, t6723: F, t829: F, t830: F, t19999: F, t20007: F, t20009: F, t20017: F, t20024: F, t20026: F, t20028: F, t20034: F, t2220: F, t2379: F, t2408: F, t2409: F, t2416: F, t2418: F, t335: F, t338: F, t353: F, t4379: F, t4427: F, t6107: F, t6739: F, t827: F, t938: F, t939: F) -> (F,) {
    let t20036 = t2376 * t6723;
    let t20038 = t829 * t830 * t20036;
    let t20043 = 7.0 / 24.0 * t19999 - t6107 * t2379 / 24.0 - t335 * t338 * t6739 * t939 / 24.0 + 7.0 / 12.0 * t20007 + 35.0 / 36.0 * t20009 + t335 * t338 * t2220 * t2418 / 8.0 + 35.0 / 72.0 * t20017 + t2408 * t2409 * t2376 * t4379 * t938 / 12.0 + 7.0 / 6.0 * t20024 + 455.0 / 324.0 * t20026 + t335 * t338 * t353 * t2416 * t20028 / 16.0 + 7.0 / 72.0 * t20034 - t827 * t20038 / 24.0 - t4427 * t2379 / 12.0;
    (t20043,)
}
