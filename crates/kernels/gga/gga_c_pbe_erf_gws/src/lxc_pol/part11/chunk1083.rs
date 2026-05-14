//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1083/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1083<F: Float>(t1076: F, t3824: F, t11630: F, t11794: F, t11592: F, t13243: F, t36869: F, t1134: F, t3772: F, t44889: F, t12041: F, t46544: F, t860: F, t1123: F, t11700: F, t13544: F, t13593: F, t15150: F, t2118: F, t2157: F, t2255: F, t2277: F, t2312: F, t36612: F, t36920: F, t3826: F, t6275: F, t6637: F, t6685: F, t9499: F) -> (F, F, F, F, F, F, F, F) {
    let t49491 = t1076 * t3824;
    let t49498 = t11794 * t11630 / 16.0;
    let t49500 = t11592 * t13243 / 6.0;
    let t49507 = 35.0 / 36.0 * t36869;
    let t49508 = t1134 * t3772;
    let t49514 = 7.0 / 4.0 * t44889;
    let t49521 = t12041 * t46544 * t860 / 24.0;
    let t49522 = 3.0 / 128.0 * t6685 * t2255 * t1123 * t49491 * t2157 - t49498 + t49500 + t2312 * t11700 * t15150 / 64.0 + t6275 * t3826 * t13544 / 16.0 - t49507 + t6637 * t9499 * t2118 * t49508 / 192.0 + 119.0 / 1152.0 * t36920 - t49514 + t2277 * t2255 * t36612 * t13593 / 256.0 + t49521;
    (t49491, t49498, t49500, t49507, t49508, t49514, t49521, t49522)
}
