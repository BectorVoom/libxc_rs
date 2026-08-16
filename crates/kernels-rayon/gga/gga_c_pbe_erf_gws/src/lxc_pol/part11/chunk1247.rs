//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1247/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1247(t274: f64, t3781: f64, t9607: f64, t1076: f64, t1123: f64, t11499: f64, t1153: f64, t13269: f64, t2118: f64, t2253: f64, t2255: f64, t2277: f64, t3257: f64, t343: f64, t3757: f64, t49491: f64, t49534: f64, t49745: f64, t49761: f64, t49763: f64, t49765: f64, t49767: f64, t49768: f64, t6275: f64, t6637: f64, t9499: f64) -> f64 {
    let t49772 = t3781 * t274;
    let t49773 = t9607 * t49772;
    let t49777 = -t2253 * t2255 * t3781 * t13269 / 128.0_f64 + t49745 + t6637 * t9499 * t2118 * t49534 / 128.0_f64 - 7.0_f64 / 384.0_f64 * t2277 * t3257 * t11499 * t3757 * t1076 - t2253 * t2255 * t1123 * t49491 * t343 / 128.0_f64 - t49761 - t49763 + t49765 + t49767 + t6275 * t1153 * t49768 / 16.0_f64 + t6275 * t1153 * t49773 / 16.0_f64;
    t49777
}
