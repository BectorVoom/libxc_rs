//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 839/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk839(t1123: f64, t13269: f64, t2255: f64, t1133: f64, t816: f64, t343: f64, t3803: f64, t3257: f64, t13233: f64, t13235: f64, t13237: f64, t13238: f64, t13240: f64, t13245: f64, t13247: f64, t13249: f64, t13254: f64, t13259: f64, t13265: f64, t2253: f64, t2312: f64) -> (f64, f64, f64) {
    let t13271 = t2255 * t1123 * t13269;
    let t13274 = t816 * t1133;
    let t13276 = t3803 * t13274 * t343;
    let t13277 = t3257 * t13276;
    let t13280 = -t13233 - t13235 - t13237 + t13238 - t13240 + t13245 + t13247 - t2253 * t13249 / 256.0_f64 - t2253 * t13254 / 256.0_f64 + t2312 * t13259 / 128.0_f64 - t2253 * t13265 / 256.0_f64 - t2253 * t13271 / 256.0_f64 - t2253 * t13277 / 128.0_f64;
    (t13271, t13277, t13280)
}
