//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 848/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk848<F: Float>(t17347: F, t17316: F, t17318: F, t17326: F, t17328: F, t17330: F, t17335: F, t17338: F, t17341: F, t17343: F, t17346: F, t1621: F, t1791: F, t5097: F, t639: F, t661: F) -> (F, F, F) {
    let t17348 = 32.0 / 45.0 * t17347;
    let t17349 = t17316 + t17318 + t17326 + t17328 + t17330 - t17335 + t17338 + t17341 - t17343 + t17346 - t17348;
    let t17354 = 16.0 / 15.0 * t639 * t1621 * t1791 * t5097 * t661;
    (t17348, t17349, t17354)
}
