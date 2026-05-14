//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 533/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk533<F: Float>(t2352: F, t898: F, t353: F, t338: F, t2118: F, t825: F, t822: F, t814: F, t830: F, t831: F, t829: F) -> (F, F, F, F, F, F) {
    let t2353 = t898 * t2352;
    let t2354 = t353 * t2353;
    let t2355 = t338 * t2354;
    let t2358 = t2118 * t825;
    let t2359 = t822 * t2358;
    let t2361 = t830 * t831 * t814;
    let t2362 = t829 * t2361;
    (t2353, t2354, t2355, t2358, t2359, t2362)
}
