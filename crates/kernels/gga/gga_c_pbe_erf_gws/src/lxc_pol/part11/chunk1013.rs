//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1013/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1013<F: Float>(t42037: F, t33149: F, t33152: F, t42050: F, t25349: F, t48291: F, t48295: F, t48299: F, t48303: F, t48305: F, t48306: F, t25354: F, t1024: F, t40790: F, t42109: F, t42131: F) -> (F, F, F, F, F, F, F, F, F) {
    let t48307 = 32.0 / 45.0 * t42037;
    let t48309 = 64.0 / 135.0 * t33149;
    let t48310 = 32.0 / 135.0 * t33152;
    let t48311 = 32.0 / 45.0 * t42050;
    let t48312 = -t48291 - t48295 + t48299 + t48303 + t48305 + t48306 + t48307 + 32.0 / 81.0 * t25349 - t48309 + t48310 - t48311;
    let t48313 = 64.0 / 405.0 * t25354;
    let t48315 = 16.0 / 15.0 * t40790 * t1024;
    let t48316 = 64.0 / 45.0 * t42109;
    let t48318 = 32.0 / 15.0 * t42131;
    (t48307, t48309, t48310, t48311, t48312, t48313, t48315, t48316, t48318)
}
