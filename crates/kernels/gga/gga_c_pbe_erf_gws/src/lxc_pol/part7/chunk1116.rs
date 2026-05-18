//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1116/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1116<F: Float>(t6729: F, t941: F, t2352: F, t6726: F, t840: F, t2376: F, t6723: F, t829: F, t830: F, t19999: F, t20007: F, t20009: F, t20017: F, t20024: F, t2220: F, t2379: F, t2408: F, t2409: F, t2416: F, t2418: F, t335: F, t338: F, t353: F, t4379: F, t4427: F, t6107: F, t6739: F, t827: F, t938: F, t939: F) -> F {
    let t20026 = t6729 * t941;
    let t20028 = t2352 * t2352;
    let t20034 = t840 * t6726;
    let t20036 = t2376 * t6723;
    let t20038 = t829 * t830 * t20036;
    let t20043 = F::new(7.0) / F::new(24.0) * t19999 - t6107 * t2379 / F::new(24.0) - t335 * t338 * t6739 * t939 / F::new(24.0) + F::new(7.0) / F::new(12.0) * t20007 + F::new(35.0) / F::new(36.0) * t20009 + t335 * t338 * t2220 * t2418 / F::new(8.0) + F::new(35.0) / F::new(72.0) * t20017 + t2408 * t2409 * t2376 * t4379 * t938 / F::new(12.0) + F::new(7.0) / F::new(6.0) * t20024 + F::new(455.0) / F::new(324.0) * t20026 + t335 * t338 * t353 * t2416 * t20028 / F::new(16.0) + F::new(7.0) / F::new(72.0) * t20034 - t827 * t20038 / F::new(24.0) - t4427 * t2379 / F::new(12.0);
    t20043
}
