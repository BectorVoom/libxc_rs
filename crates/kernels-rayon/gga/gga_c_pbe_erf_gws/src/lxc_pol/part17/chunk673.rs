//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 673/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk673(t816: f64, t820: f64, t3258: f64, t3257: f64, t2277: f64, t2290: f64, t2312: f64, t3110: f64, t3115: f64, t3118: f64, t3122: f64, t3127: f64, t3182: f64, t3186: f64, t3247: f64, t3249: f64, t3253: f64) -> (f64, f64, f64, f64) {
    let t3259 = t816 * t820;
    let t3260 = t3258 * t3259;
    let t3261 = t3257 * t3260;
    let t3265 = t3247 * t3249 / 512.0_f64 - t3118 + t3110 - t2312 * t3253 / 384.0_f64 + t2277 * t3261 / 768.0_f64 + t3186 + t3115 + t3122 - t3127 - t3182 + 7.0_f64 / 2304.0_f64 * t2290;
    (t3259, t3260, t3261, t3265)
}
