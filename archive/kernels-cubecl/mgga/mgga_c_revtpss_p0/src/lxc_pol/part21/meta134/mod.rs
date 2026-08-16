//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta134 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk863;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk864;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk865;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk866;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk867;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk868;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk869;
use chunk7::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk870;
use chunk8::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk871;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta134<F: Float>(t3152: F, t3162: F, t1042: F, t1038: F, t1052: F, t1036: F, t1033: F, t127: F, t246: F, t1046: F, t1041: F, t1066: F, t2862: F, t247: F, t283: F, t905: F, t66: F, t2853: F, t1047: F, t1063: F, t1068: F, t3082: F, t3083: F, t3086: F, t3091: F, t3097: F, t3101: F, t3106: F, t3112: F, t3115: F, t3120: F, t3124: F, t3127: F, t3130: F, t3136: F, t3150: F, t3157: F, t3161: F, t348: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t3163, t3164) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk863::<F>(t3152, t3162, t1042);
        let t3168 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk864::<F>(t1038, t1052, t1036);
        let t3169 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk865::<F>(t1033, t3168);
        let t3172 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk866::<F>(t127, t246);
        let t3173 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk867::<F>(t1046, t3172);
        let (t3174, t3177) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk868::<F>(t1041, t3173, t1066, t2862, t247);
        let t3181 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk869::<F>(t283, t905);
        let (t3182, t3184) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk870::<F>(t3181, t66, t2853, t247);
        let t3187 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk871::<F>(t1041, t1047, t1063, t1068, t3082, t3083, t3086, t3091, t3097, t3101, t3106, t3112, t3115, t3120, t3124, t3127, t3130, t3136, t3150, t3157, t3161, t3164, t3169, t3174, t3177, t3184, t348);
    (t3163, t3164, t3168, t3169, t3172, t3173, t3174, t3177, t3181, t3182, t3184, t3187)
}
