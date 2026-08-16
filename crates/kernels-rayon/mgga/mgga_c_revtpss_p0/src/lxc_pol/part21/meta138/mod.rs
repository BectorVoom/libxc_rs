//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta138 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk884;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk885;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk886;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk887;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk888;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk889;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk890;
use chunk7::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk891;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta138(t2852: f64, t3252: f64, t2251: f64, t1012: f64, t1011: f64, t1017: f64, t1025: f64, t1028: f64, t1068: f64, t3188: f64, t3191: f64, t3194: f64, t3197: f64, t3203: f64, t3205: f64, t3208: f64, t3211: f64, t3216: f64, t3220: f64, t3224: f64, t3231: f64, t3234: f64, t3238: f64, t3241: f64, t3245: f64, t3248: f64, t375: f64, t3187: f64, t225: f64, t385: f64, t1071: f64, t342: f64, t1077: f64, t384: f64, t1096: f64, t1086: f64, t989: f64, t1082: f64, t3059: f64, t378: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3254, t3255, t3258) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk884(t2852, t3252, t2251, t1012, t1011, t1017, t1025, t1028, t1068, t3188, t3191, t3194, t3197, t3203, t3205, t3208, t3211, t3216, t3220, t3224, t3231, t3234, t3238, t3241, t3245, t3248, t375);
        let t3259 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk885(t3187, t3258);
        let (t3261, t3264) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk886(t225, t3259, t385, t1071, t342);
        let (t3268, t3269) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk887(t1077, t384, t225);
        let t3270 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk888(t1096);
        let t3271 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk889(t3269, t3270);
        let t3278 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk890(t1086, t989);
        let (t3283, t3286) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk891(t1082, t3059, t1086, t378);
    (t3254, t3255, t3259, t3261, t3264, t3268, t3269, t3270, t3271, t3278, t3283, t3286)
}
