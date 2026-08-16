//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1234/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1234(t1123: f64, t11700: f64, t13544: f64, t13593: f64, t15150: f64, t2118: f64, t2157: f64, t2255: f64, t2277: f64, t2312: f64, t36612: f64, t36920: f64, t3826: f64, t49491: f64, t49498: f64, t49500: f64, t49507: f64, t49508: f64, t49514: f64, t49521: f64, t6275: f64, t6637: f64, t6685: f64, t9499: f64) -> f64 {
    let t49522 = 3.0_f64 / 128.0_f64 * t6685 * t2255 * t1123 * t49491 * t2157 - t49498 + t49500 + t2312 * t11700 * t15150 / 64.0_f64 + t6275 * t3826 * t13544 / 16.0_f64 - t49507 + t6637 * t9499 * t2118 * t49508 / 192.0_f64 + 119.0_f64 / 1152.0_f64 * t36920 - t49514 + t2277 * t2255 * t36612 * t13593 / 256.0_f64 + t49521;
    t49522
}
