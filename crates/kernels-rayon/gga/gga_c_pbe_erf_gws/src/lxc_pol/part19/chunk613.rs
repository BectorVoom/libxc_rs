//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 613/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk613(t3219: f64, t3235: f64, t875: f64, t1105: f64, t6: f64, t2345: f64, t2253: f64, t2343: f64, t3125: f64, t3130: f64, t3136: f64, t3144: f64, t3150: f64, t3176: f64, t3224: f64, t3228: f64, t3232: f64) -> (f64, f64, f64, f64) {
    let t3237 = t3235 * t3219 * t875;
    let t3240 = t6 * t1105;
    let t3242 = t2345 * t3240 * t875;
    let t3245 = -t2253 * t3224 / 768.0_f64 + t3144 + t3150 - t3125 + t3176 - t3136 - t2253 * t3228 / 768.0_f64 - t3130 + t2343 * t3232 / 384.0_f64 - t2343 * t3237 / 1536.0_f64 + t2343 * t3242 / 384.0_f64;
    (t3237, t3240, t3242, t3245)
}
