//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 955/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk955<F: Float>(t1105: F, t2395: F, t2370: F, t830: F, t2494: F, t831: F, t2358: F, t3039: F, t1114: F, t4409: F, t2362: F, t2388: F, t2392: F, t2408: F, t3052: F, t3055: F, t3066: F, t335: F, t4459: F, t4464: F, t6778: F, t6833: F, t827: F, t9323: F, t9328: F, t9691: F, t9695: F, t9697: F, t9701: F, t9704: F, t9709: F) -> (F, F, F) {
    let t9716 = t2395 * t1105;
    let t9718 = t2370 * t830 * t9716;
    let t9721 = t831 * t2494;
    let t9723 = t2370 * t830 * t9721;
    let t9726 = t3039 * t2358;
    let t9729 = t1114 * t4409;
    let t9737 = t3066 * t9323 / 48.0 + t2408 * t9328 / 48.0 - t335 * t9691 / 96.0 + t9695 + t9697 * t6778 / 32.0 - t9701 + t2408 * t9704 / 24.0 - t827 * t9709 / 48.0 - t2388 * t3052 / 48.0 - t2392 * t3052 / 48.0 - t827 * t9718 / 24.0 - t827 * t9723 / 24.0 - t9726 * t2362 / 48.0 - t9729 * t2362 / 48.0 - t3055 * t4459 / 48.0 - t3055 * t4464 / 96.0 - 7.0 / 48.0 * t6833;
    (t9716, t9721, t9737)
}
