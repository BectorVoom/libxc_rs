//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 727/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk727<F: Float>(t2253: F, t2343: F, t3247: F, t6246: F, t6251: F, t6255: F, t6260: F, t6262: F, t6266: F, t6273: F, t6275: F, t6279: F, t6284: F, t6289: F, t6293: F, t6299: F, t6305: F, t6310: F, t902: F) -> (F,) {
    let t6313 = -t6246 + t6251 - t6255 - t6260 + t902 * t6262 / 768.0 + t902 * t6266 / 1536.0 + t6273 + t6275 * t6279 / 32.0 + t2343 * t6284 / 128.0 - 3.0 / 128.0 * t3247 * t6289 - t2253 * t6293 / 256.0 - t2253 * t6299 / 256.0 - t2253 * t6305 / 256.0 - t2343 * t6310 / 512.0;
    (t6313,)
}
