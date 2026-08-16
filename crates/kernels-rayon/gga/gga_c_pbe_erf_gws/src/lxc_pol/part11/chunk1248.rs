//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1248/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1248(t45546: f64, t37938: f64, t1109: f64, t3835: f64, t3128: f64, t45487: f64, t11668: f64, t13491: f64, t1076: f64, t13290: f64, t13385: f64, t13534: f64, t2118: f64, t2253: f64, t2255: f64, t2277: f64, t2312: f64, t3258: f64, t3763: f64, t3772: f64, t3781: f64, t37814: f64, t45568: f64, t9441: f64, t9499: f64, t9637: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t49792 = 7.0_f64 / 3.0_f64 * t45546;
    let t49793 = 35.0_f64 / 72.0_f64 * t37938;
    let t49794 = t3835 * t1109;
    let t49800 = t3128 * t45487;
    let t49802 = t11668 * t13491 / 32.0_f64;
    let t49808 = -119.0_f64 / 288.0_f64 * t37814 - t2312 * t2255 * t3781 * t13385 / 48.0_f64 + t2277 * t2255 * t9441 * t1076 * t13290 / 256.0_f64 - t2253 * t2255 * t13534 * t3763 / 192.0_f64 + t49792 - t49793 - 3.0_f64 / 128.0_f64 * t9637 * t9499 * t2118 * t49794 - 7.0_f64 / 12.0_f64 * t45568 - t49800 - t49802 - t2277 * t2255 * t3258 * t1076 * t3772 / 512.0_f64;
    (t49792, t49793, t49794, t49800, t49802, t49808)
}
