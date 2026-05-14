//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 962/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk962<F: Float>(t12042: F, t12045: F, t1076: F, t814: F, t2255: F, t3258: F, t12005: F, t12009: F, t12015: F, t12021: F, t12025: F, t12031: F, t12034: F, t12038: F, t12040: F, t2253: F, t2277: F, t6579: F, t9645: F, t9658: F) -> (F, F, F) {
    let t12047 = t12042 * t12045 / 48.0;
    let t12048 = t1076 * t814;
    let t12050 = t2255 * t3258 * t12048;
    let t12053 = -t9645 - t12005 - t2253 * t12009 / 384.0 - t2253 * t12015 / 768.0 - t2253 * t12021 / 384.0 + 5.0 / 384.0 * t6579 * t12025 - 119.0 / 1728.0 * t9658 - t12031 + t12034 + t12038 - t12040 + t12047 - t2277 * t12050 / 1536.0;
    (t12047, t12050, t12053)
}
