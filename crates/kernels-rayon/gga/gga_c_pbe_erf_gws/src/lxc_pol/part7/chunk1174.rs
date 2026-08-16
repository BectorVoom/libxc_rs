//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1174/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1174(t20269: f64, t2276: f64, t932: f64, t2315: f64, t6369: f64, t6627: f64, t2323: f64, t6387: f64, t19505: f64, t20919: f64, t20921: f64, t20926: f64, t20932: f64, t20934: f64, t20941: f64, t20945: f64, t2113: f64, t2255: f64, t2300: f64, t2312: f64, t904: f64, t916: f64, t929: f64, t9465: f64) -> f64 {
    let t20948 = t2276 * t20269 * t932;
    let t20949 = t20948 * t2315;
    let t20951 = t6627 * t6369;
    let t20953 = t2323 * t6387;
    let t20959 = t20919 - t20921 - t2312 * t2255 * t2113 * t9465 / 64.0_f64 - 119.0_f64 / 1152.0_f64 * t20926 + 5.0_f64 / 128.0_f64 * t20932 * t916 * t904 * t20934 + 119.0_f64 / 384.0_f64 * t20941 - 35.0_f64 / 48.0_f64 * t20945 - 119.0_f64 / 144.0_f64 * t20949 + 35.0_f64 / 48.0_f64 * t20951 + 35.0_f64 / 48.0_f64 * t20953 + 5.0_f64 / 256.0_f64 * t929 * t2300 * t904 * t19505;
    t20959
}
