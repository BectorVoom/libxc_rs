//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta140 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk899;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk900;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk901;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk902;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk903;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk904;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta140<F: Float>(t3300: F, t3304: F, t1043: F, t1071: F, t1089: F, t3133: F, t378: F, t1035: F, t3140: F, t342: F, t3303: F, t357: F, t3259: F, t380: F, t1024: F, t1083: F, t1087: F, t1090: F, t1093: F, t3043: F, t3204: F, t3223: F, t3278: F, t3283: F, t3287: F, t3288: F, t3292: F, t3295: F, t3299: F, t381: F, t989: F, t1079: F, t1000: F, t1073: F, t1076: F, t1097: F, t3047: F, t3052: F, t3058: F, t3060: F, t3063: F, t3067: F, t3076: F, t3261: F, t3264: F, t3271: F, t386: F, t995: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t3305, t3309, t3313, t3316) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk899::<F>(t3300, t3304, t1043, t1071, t1089, t3133, t378, t1035, t3140);
        let t3317 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk900::<F>(t3316, t342);
        let t3318 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk901::<F>(t3303, t357);
        let (t3319, t3322, t3325) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk902::<F>(t3300, t3318, t3259, t380, t1024, t1083, t1087, t1090, t1093, t3043, t3204, t3223, t3278, t3283, t3287, t3288, t3292, t3295, t3299, t3305, t3309, t3313, t3317, t342, t381, t989);
        let t3326 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk903::<F>(t1079, t3325);
        let t3329 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk904::<F>(t1000, t1073, t1076, t1097, t3043, t3047, t3052, t3058, t3060, t3063, t3067, t3076, t3261, t3264, t3271, t3326, t342, t386, t989, t995);
    (t3305, t3309, t3313, t3316, t3317, t3318, t3319, t3322, t3325, t3326, t3329)
}
