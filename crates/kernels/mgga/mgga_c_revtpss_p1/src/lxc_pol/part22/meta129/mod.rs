//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta129 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk864;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk865;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk866;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk867;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk868;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk869;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk870;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk871;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta129<F: Float>(t1043: F, t73: F, t357: F, t905: F, t606: F, t3092: F, t1066: F, t2858: F, t247: F, t1052: F, t369: F, t361: F, t351: F, t1065: F, t126: F, t906: F, t1063: F, t1086: F, t994: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t3093, t3094) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk864::<F>(t1043, t73, t357, t905);
        let t3095 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk865::<F>(t3094, t606);
        let t3096 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk866::<F>(t3093, t3095);
        let (t3097, t3101, t3105) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk867::<F>(t3092, t3096, t1066, t2858, t247, t1052, t369, t361);
        let t3106 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk868::<F>(t3105, t351);
        let t3109 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk869::<F>(t1065, t126);
        let t3111 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk870::<F>(t3109, t906, t247);
        let (t3112, t3114) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk871::<F>(t1063, t3111, t1086, t994);
    (t3093, t3094, t3095, t3096, t3097, t3101, t3105, t3106, t3109, t3111, t3112, t3114)
}
