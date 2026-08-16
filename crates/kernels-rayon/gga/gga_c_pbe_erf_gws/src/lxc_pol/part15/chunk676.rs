//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 676/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk676(t3294: f64, t905: f64, t1113: f64, t2271: f64, t1154: f64, t2289: f64, t2277: f64, t2312: f64, t3146: f64, t3170: f64, t3177: f64, t3193: f64, t3279: f64, t3283: f64, t3287: f64, t3291: f64, t902: f64, t914: f64, t929: f64) -> (f64, f64, f64, f64) {
    let t3295 = t905 * t3294;
    let t3298 = t1113 * t2271;
    let t3299 = t905 * t3298;
    let t3302 = t2289 * t1154;
    let t3304 = 5.0_f64 / 768.0_f64 * t929 * t3279 - t914 * t3283 / 1536.0_f64 + t3177 - t2312 * t3287 / 384.0_f64 - t2277 * t3291 / 1536.0_f64 + t902 * t3295 / 1536.0_f64 + t902 * t3299 / 1536.0_f64 + t3193 - t3170 + t3146 + 7.0_f64 / 2304.0_f64 * t3302;
    (t3295, t3298, t3299, t3304)
}
