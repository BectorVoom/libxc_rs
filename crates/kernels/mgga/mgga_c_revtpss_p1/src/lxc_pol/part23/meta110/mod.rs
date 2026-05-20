//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta110 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk717;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk718;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk719;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk720;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk721;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk722;
use chunk6::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk723;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta110<F: Float>(t2798: F, t2801: F, t72: F, t860: F, t686: F, t874: F, t2470: F, t875: F, t251: F, t2718: F, t822: F, t1941: F, t268: F, t271: F, t689: F, t907: F, t1065: F, t159: F, t631: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t2802, t2804, t2806, t2810, t2811) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk717::<F>(t2798, t2801, t72, t860, t686, t874, t2470, t875, t251, t2718);
        let t2815 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk718::<F>(t822, t860);
        let t2846 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk719::<F>(t1941, t268, t271);
        let (t2847, t2848) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk720::<F>(t2846, t689, t907);
        let t2850 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk721::<F>(t1065, t159);
        let t2851 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk722::<F>(t631);
        let t2852 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk723::<F>(t2851);
    (t2802, t2804, t2806, t2810, t2811, t2815, t2846, t2847, t2848, t2850, t2851, t2852)
}
