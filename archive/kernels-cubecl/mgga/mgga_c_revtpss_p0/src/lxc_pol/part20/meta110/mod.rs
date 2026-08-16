//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta110 (260520-c91 hierarchical CSE).
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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk631;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk632;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk633;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk634;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk635;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk636;
use chunk6::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk637;
use chunk7::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk638;
use chunk8::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk639;
use chunk9::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk640;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta110<F: Float>(t357: F, t905: F, t606: F, t3093: F, t3092: F, t1066: F, t2858: F, t247: F, t1052: F, t369: F, t361: F, t351: F, t1065: F, t126: F, t906: F, t1063: F, t1086: F, t994: F, t3090: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t3094, t3095) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk631::<F>(t357, t905, t606);
        let t3096 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk632::<F>(t3093, t3095);
        let t3097 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk633::<F>(t3092, t3096);
        let t3101 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk634::<F>(t1066, t2858, t247);
        let t3105 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk635::<F>(t1052, t369, t361);
        let t3106 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk636::<F>(t3105, t351);
        let t3109 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk637::<F>(t1065, t126);
        let t3111 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk638::<F>(t3109, t906, t247);
        let (t3112, t3114) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk639::<F>(t1063, t3111, t1086, t994);
        let t3115 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk640::<F>(t3090, t3114);
    (t3094, t3095, t3096, t3097, t3101, t3105, t3106, t3109, t3111, t3112, t3114, t3115)
}
