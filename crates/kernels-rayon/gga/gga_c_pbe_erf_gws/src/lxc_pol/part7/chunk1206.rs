//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1206/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1206(t21430: f64, t2281: f64, t20695: f64, t274: f64, t20432: f64, t6328: f64, t8782: f64, t19561: f64, t20527: f64, t20571: f64, t20708: f64, t21399: f64, t21400: f64, t21405: f64, t21412: f64, t21414: f64, t21424: f64, t21429: f64, t2255: f64, t2277: f64, t2345: f64, t254: f64, t3247: f64, t3257: f64, t6275: f64, t6282: f64, t820: f64, t906: f64, t9568: f64) -> (f64, f64) {
    let t21431 = t21430 * t2281;
    let t21438 = t274 * t20695;
    let t21445 = t8782 * t20432 * t6328 / 16.0_f64;
    let t21446 = -5.0_f64 / 16.0_f64 * t21399 * t254 * t21400 * t906 + t6275 * t20527 * t21405 / 8.0_f64 + t21412 - t21414 - 7.0_f64 / 384.0_f64 * t2277 * t3257 * t20571 * t9568 - t21424 + t21429 - 119.0_f64 / 1152.0_f64 * t21431 - 3.0_f64 / 64.0_f64 * t3247 * t2345 * t6282 * t20708 - t2277 * t2255 * t820 * t19561 * t21438 / 256.0_f64 + t21445;
    (t21445, t21446)
}
