//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1232/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1232(t1105: f64, t1123: f64, t13534: f64, t13578: f64, t2255: f64, t2312: f64, t2345: f64, t3235: f64, t3247: f64, t3373: f64, t36803: f64, t3752: f64, t37829: f64, t44282: f64, t44710: f64, t49415: f64, t49464: f64, t49471: f64, t49472: f64, t49478: f64, t49483: f64, t824: f64, t8884: f64, t902: f64, t905: f64, t9425: f64, t9482: f64) -> f64 {
    let t49489 = -t2312 * t2255 * t1123 * t3373 * t1105 / 96.0_f64 - t2312 * t9482 * t13578 * t37829 / 24.0_f64 - t2312 * t2255 * t13534 * t3752 / 96.0_f64 + t49415 + t902 * t905 * t49464 * t824 / 1536.0_f64 - t49471 - t49472 - t49478 + t3247 * t3235 * t44710 * t8884 / 128.0_f64 + t9425 * t2345 * t44282 * t49483 / 8.0_f64 + 119.0_f64 / 576.0_f64 * t36803;
    t49489
}
