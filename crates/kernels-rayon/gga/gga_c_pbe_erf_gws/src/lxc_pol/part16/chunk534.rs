//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 534/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk534(t2289: f64, t918: f64, t2195: f64, t904: f64, t916: f64, t2089: f64, t2117: f64, t2126: f64, t2140: f64, t2162: f64, t2166: f64, t2194: f64, t2199: f64, t2204: f64, t2253: f64, t2259: f64, t2266: f64, t2268: f64, t2273: f64, t2277: f64, t2281: f64, t2285: f64, t902: f64, t914: f64) -> (f64, f64, f64) {
    let t2290 = t2289 * t918;
    let t2292 = t904 * t2195;
    let t2293 = t916 * t2292;
    let t2296 = -t2140 - t2253 * t2259 / 384.0_f64 + t2266 * t2268 / 512.0_f64 - t2126 + t2166 - t2194 - t2199 + t902 * t2273 / 1536.0_f64 - t2277 * t2281 / 1536.0_f64 + t2162 + t2089 + t2117 - t914 * t2285 / 1536.0_f64 + 7.0_f64 / 1152.0_f64 * t2290 - t914 * t2293 / 1536.0_f64 + t2204;
    (t2290, t2293, t2296)
}
