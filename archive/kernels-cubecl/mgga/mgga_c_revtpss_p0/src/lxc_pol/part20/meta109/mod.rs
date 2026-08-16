//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta109 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk623;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk624;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk625;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk626;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk627;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk628;
use chunk6::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk629;
use chunk7::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk630;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta109<F: Float>(t2846: F, t2848: F, t2855: F, t2860: F, t2864: F, t996: F, t221: F, t346: F, t696: F, t345: F, t2270: F, t344: F, t1003: F, t1007: F, t360: F, t365: F, t1038: F, t72: F, t1087: F, t1066: F, t828: F, t1043: F, t73: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t3075 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk623::<F>(t2846, t2848, t2855, t2860, t2864);
        let t3076 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk624::<F>(t3075, t996);
        let (t3080, t3082, t3083, t3086, t3088) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk625::<F>(t221, t346, t696, t345, t2270, t344, t1003, t1007, t360, t365);
        let t3089 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk626::<F>(t1038, t72);
        let t3090 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk627::<F>(t3088, t3089);
        let t3091 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk628::<F>(t1087, t3090);
        let t3092 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk629::<F>(t1066, t828);
        let t3093 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk630::<F>(t1043, t73);
    (t3075, t3076, t3080, t3082, t3083, t3086, t3088, t3089, t3090, t3091, t3092, t3093)
}
