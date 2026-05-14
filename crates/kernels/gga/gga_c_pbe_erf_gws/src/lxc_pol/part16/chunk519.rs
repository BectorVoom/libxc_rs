//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 519/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk519<F: Float>(t2289: F, t918: F, t2195: F, t904: F, t916: F, t2089: F, t2117: F, t2126: F, t2140: F, t2162: F, t2166: F, t2194: F, t2199: F, t2204: F, t2253: F, t2259: F, t2266: F, t2268: F, t2273: F, t2277: F, t2281: F, t2285: F, t902: F, t914: F) -> (F, F, F) {
    let t2290 = t2289 * t918;
    let t2292 = t904 * t2195;
    let t2293 = t916 * t2292;
    let t2296 = -t2140 - t2253 * t2259 / 384.0 + t2266 * t2268 / 512.0 - t2126 + t2166 - t2194 - t2199 + t902 * t2273 / 1536.0 - t2277 * t2281 / 1536.0 + t2162 + t2089 + t2117 - t914 * t2285 / 1536.0 + 7.0 / 1152.0 * t2290 - t914 * t2293 / 1536.0 + t2204;
    (t2290, t2293, t2296)
}
