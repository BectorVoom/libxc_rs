//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1159/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1159(t20306: f64, t20669: f64, t20670: f64, t20676: f64, t20682: f64, t20687: f64, t20691: f64, t20700: f64, t20702: f64, t20703: f64, t20712: f64, t2084: f64, t2253: f64, t2255: f64, t2277: f64, t2278: f64, t2312: f64, t3223: f64, t3257: f64, t6195: f64) -> f64 {
    let t20713 = t20669 + t2312 * t2255 * t2278 * t20670 / 64.0_f64 - 7.0_f64 / 32.0_f64 * t20676 - t2253 * t20306 * t3223 / 192.0_f64 - t2253 * t3257 * t2084 * t20682 / 64.0_f64 + 7.0_f64 / 48.0_f64 * t20687 - t20691 - t20700 + t20702 - t2277 * t3257 * t6195 * t20703 / 192.0_f64 - t20712;
    t20713
}
