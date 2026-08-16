//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta137 (260520-c91 hierarchical CSE).
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
mod chunk9;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk912;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk913;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk914;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk915;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk916;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk917;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk918;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk919;
use chunk8::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk920;
use chunk9::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk921;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta137(t225: f64, t3259: f64, t385: f64, t1071: f64, t342: f64, t1077: f64, t384: f64, t1096: f64, t1086: f64, t989: f64, t1082: f64, t3059: f64, t378: f64, t994: f64, t1089: f64, t3118: f64, t359: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3261, t3264) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk912(t225, t3259, t385, t1071, t342);
        let t3268 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk913(t1077, t384);
        let t3269 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk914(t225, t3268);
        let t3270 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk915(t1096);
        let t3271 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk916(t3269, t3270);
        let t3278 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk917(t1086, t989);
        let (t3283, t3286) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk918(t1082, t3059, t1086, t378);
        let t3287 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk919(t3286, t994);
        let t3288 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk920(t1089, t3118);
        let t3291 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk921(t1071, t359);
    (t3261, t3264, t3268, t3269, t3270, t3271, t3278, t3283, t3286, t3287, t3288, t3291)
}
