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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta134(t3152: f64, t3162: f64, t1042: f64, t1038: f64, t1052: f64, t1036: f64, t1033: f64, t127: f64, t246: f64, t1046: f64, t1041: f64, t1066: f64, t2862: f64, t247: f64, t283: f64, t905: f64, t66: f64, t2853: f64, t1047: f64, t1063: f64, t1068: f64, t3082: f64, t3083: f64, t3086: f64, t3091: f64, t3097: f64, t3101: f64, t3106: f64, t3112: f64, t3115: f64, t3120: f64, t3124: f64, t3127: f64, t3130: f64, t3136: f64, t3150: f64, t3157: f64, t3161: f64, t348: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3163, t3164) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk863(t3152, t3162, t1042);
        let t3168 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk864(t1038, t1052, t1036);
        let t3169 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk865(t1033, t3168);
        let t3172 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk866(t127, t246);
        let t3173 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk867(t1046, t3172);
        let (t3174, t3177) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk868(t1041, t3173, t1066, t2862, t247);
        let t3181 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk869(t283, t905);
        let (t3182, t3184) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk870(t3181, t66, t2853, t247);
        let t3187 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk871(t1041, t1047, t1063, t1068, t3082, t3083, t3086, t3091, t3097, t3101, t3106, t3112, t3115, t3120, t3124, t3127, t3130, t3136, t3150, t3157, t3161, t3164, t3169, t3174, t3177, t3184, t348);
    (t3163, t3164, t3168, t3169, t3172, t3173, t3174, t3177, t3181, t3182, t3184, t3187)
}
