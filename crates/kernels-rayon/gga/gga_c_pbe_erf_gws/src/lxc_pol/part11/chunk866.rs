//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 866/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk866(t13585: f64, t905: f64, t13335: f64, t904: f64, t916: f64, t13290: f64, t274: f64, t2255: f64, t9441: f64, t13369: f64, t12092: f64, t12057: f64, t12061: f64, t13569: f64, t13571: f64, t13575: f64, t13582: f64, t13583: f64, t2266: f64, t2277: f64, t2312: f64, t902: f64, t914: f64, t9658: f64, t9669: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13586 = t905 * t13585;
    let t13590 = t916 * t904 * t13335;
    let t13593 = t274 * t13290;
    let t13595 = t2255 * t9441 * t13593;
    let t13599 = t916 * t904 * t13369;
    let t13602 = 7.0_f64 / 24.0_f64 * t12092;
    let t13603 = -t13569 - t2312 * t13571 / 64.0_f64 - 119.0_f64 / 1152.0_f64 * t9658 - t13575 + 7.0_f64 / 768.0_f64 * t12057 + 119.0_f64 / 2304.0_f64 * t9669 + t13582 + t13583 - 7.0_f64 / 768.0_f64 * t12061 + t902 * t13586 / 1536.0_f64 - t914 * t13590 / 1536.0_f64 + t2277 * t13595 / 768.0_f64 + 3.0_f64 / 512.0_f64 * t2266 * t13599 - t13602;
    (t13586, t13590, t13593, t13595, t13599, t13602, t13603)
}
