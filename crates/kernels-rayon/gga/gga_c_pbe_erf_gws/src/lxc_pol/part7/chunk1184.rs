//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1184/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1184(t2298: f64, t814: f64, t322: f64, t6382: f64, t2182: f64, t19505: f64, t19553: f64, t2074: f64, t20992: f64, t20995: f64, t20998: f64, t2178: f64, t2181: f64, t2183: f64, t2186: f64, t339: f64, t340: f64, t4379: f64, t6421: f64, t6424: f64, t6429: f64, t6430: f64, t6433: f64, t6436: f64, t870: f64, t871: f64) -> (f64, f64) {
    let t21003 = t814 * t2298;
    let t21010 = t322 * t6382;
    let t21011 = t2182 * t2182;
    let t21027 = -36.0_f64 * t19505 * t2181 * t339 + 3.0_f64 * t19553 * t339 * t870 + 360.0_f64 * t2074 * t2183 * t6429 - t20992 * t339 * t340 - 360.0_f64 * t21010 * t21011 * t339 - 48.0_f64 * t2181 * t4379 * t871 + 12.0_f64 * t20995 * t871 - 72.0_f64 * t20998 * t2183 + 240.0_f64 * t21003 * t6430 + 12.0_f64 * t2178 * t6436 + 18.0_f64 * t2186 * t6421 - 144.0_f64 * t6424 * t6433;
    (t21011, t21027)
}
