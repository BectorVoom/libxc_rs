//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta140 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk899;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk900;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk901;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk902;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk903;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk904;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta140(t3300: f64, t3304: f64, t1043: f64, t1071: f64, t1089: f64, t3133: f64, t378: f64, t1035: f64, t3140: f64, t342: f64, t3303: f64, t357: f64, t3259: f64, t380: f64, t1024: f64, t1083: f64, t1087: f64, t1090: f64, t1093: f64, t3043: f64, t3204: f64, t3223: f64, t3278: f64, t3283: f64, t3287: f64, t3288: f64, t3292: f64, t3295: f64, t3299: f64, t381: f64, t989: f64, t1079: f64, t1000: f64, t1073: f64, t1076: f64, t1097: f64, t3047: f64, t3052: f64, t3058: f64, t3060: f64, t3063: f64, t3067: f64, t3076: f64, t3261: f64, t3264: f64, t3271: f64, t386: f64, t995: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3305, t3309, t3313, t3316) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk899(t3300, t3304, t1043, t1071, t1089, t3133, t378, t1035, t3140);
        let t3317 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk900(t3316, t342);
        let t3318 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk901(t3303, t357);
        let (t3319, t3322, t3325) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk902(t3300, t3318, t3259, t380, t1024, t1083, t1087, t1090, t1093, t3043, t3204, t3223, t3278, t3283, t3287, t3288, t3292, t3295, t3299, t3305, t3309, t3313, t3317, t342, t381, t989);
        let t3326 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk903(t1079, t3325);
        let t3329 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk904(t1000, t1073, t1076, t1097, t3043, t3047, t3052, t3058, t3060, t3063, t3067, t3076, t3261, t3264, t3271, t3326, t342, t386, t989, t995);
    (t3305, t3309, t3313, t3316, t3317, t3318, t3319, t3322, t3325, t3326, t3329)
}
