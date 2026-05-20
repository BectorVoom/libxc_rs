//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta117 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk760;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk761;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk762;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk763;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk764;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk765;
use chunk6::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk766;
use chunk7::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk767;
use chunk8::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk768;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta117<F: Float>(t1052: F, t369: F, t361: F, t351: F, t1065: F, t126: F, t906: F, t247: F, t1063: F, t1086: F, t994: F, t3090: F, t373: F, t66: F, t828: F, t1032: F, t989: F, t1040: F, t1024: F, t1062: F, t1031: F, t196: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t3105 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk760::<F>(t1052, t369, t361);
        let t3106 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk761::<F>(t3105, t351);
        let t3109 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk762::<F>(t1065, t126);
        let (t3111, t3112, t3114, t3115) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk763::<F>(t3109, t906, t247, t1063, t1086, t994, t3090);
        let t3116 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk764::<F>(t373, t66);
        let t3117 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk765::<F>(t3116, t828);
        let t3124 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk766::<F>(t1032, t989, t1040);
        let t3127 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk767::<F>(t1024, t1062);
        let t3140 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk768::<F>(t1031, t196);
    (t3105, t3106, t3109, t3111, t3112, t3114, t3115, t3116, t3117, t3124, t3127, t3140)
}
