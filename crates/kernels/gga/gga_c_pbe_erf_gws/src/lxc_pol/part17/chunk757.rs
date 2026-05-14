//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 757/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk757<F: Float>(t2276: F, t6401: F, t2281: F, t2100: F, t369: F, t814: F, t931: F, t2298: F, t322: F, t2164: F, t2197: F, t2192: F, t2331: F, t899: F, t912: F, t918: F) -> (F, F, F, F, F, F, F, F, F) {
    let t6416 = t2276 * t6401;
    let t6417 = t6416 * t2281;
    let t6421 = t2100 * t369;
    let t6424 = t814 * t931;
    let t6429 = t322 * t2298;
    let t6445 = t2164 * t2197;
    let t6447 = t2164 * t2192;
    let t6455 = t899 * t912 * t2331;
    let t6456 = t6455 * t918;
    (t6416, t6417, t6421, t6424, t6429, t6445, t6447, t6455, t6456)
}
