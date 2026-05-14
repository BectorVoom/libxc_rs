//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 930/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk930<F: Float>(t6365: F, t904: F, t8891: F, t1123: F, t6297: F, t2255: F, t2253: F, t2277: F, t2343: F, t6685: F, t8821: F, t8823: F, t8826: F, t8831: F, t8832: F, t8835: F, t8839: F, t9334: F, t9338: F, t9342: F) -> (F, F, F, F) {
    let t9343 = t6365 * t904;
    let t9344 = t9343 * t8891;
    let t9347 = t1123 * t6297;
    let t9348 = t2255 * t9347;
    let t9351 = t2277 * t9334 / 256.0 + t6685 * t9338 / 256.0 - t8821 + t8823 + t9342 + t8826 + t8831 - 5.0 / 192.0 * t2343 * t9344 + t8832 - t2253 * t9348 / 384.0 + t8835 - t8839;
    (t9344, t9347, t9348, t9351)
}
