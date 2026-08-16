//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta116 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk783;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk784;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk785;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk786;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk787;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk788;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk789;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk790;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta116<F: Float>(t2797: F, t786: F, t231: F, t268: F, t675: F, t836: F, t72: F, t860: F, t686: F, t874: F, t2470: F, t875: F, t251: F, t2718: F, t822: F, t213: F, t234: F, t2646: F, t2724: F, t2754: F, t2760: F, t2776: F, t2780: F, t2787: F, t2791: F, t2796: F, t820: F, t837: F, t879: F, t868: F, t2437: F, t2443: F, t2446: F, t2449: F, t2460: F, t2462: F, t2468: F, t2473: F, t257: F, t2761: F, t2765: F, t2772: F, t865: F, t887: F, t198: F, t207: F, t2392: F, t2393: F, t2394: F, t2400: F, t2402: F, t2403: F, t2404: F, t2408: F, t2411: F, t2416: F, t2430: F, t2569: F, t2614: F, t2617: F, t765: F, t775: F, t892: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t2798 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk783::<F>(t2797, t786);
        let t2801 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk784::<F>(t231, t268, t675, t836);
        let (t2802, t2804, t2806, t2810, t2811) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk785::<F>(t2798, t2801, t72, t860, t686, t874, t2470, t875, t251, t2718);
        let t2815 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk786::<F>(t822, t860);
        let t2828 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk787::<F>(t213, t234, t2646, t2724, t2754, t2760, t2776, t2780, t2787, t2791, t2796, t2802, t2806, t2810, t2811, t2815, t820, t837, t879);
        let t2829 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk788::<F>(t2828, t868);
        let t2832 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk789::<F>(t213, t2437, t2443, t2446, t2449, t2460, t2462, t2468, t2473, t257, t2761, t2765, t2772, t2829, t865, t887);
        let t2836 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk790::<F>(t198, t207, t2392, t2393, t2394, t2400, t2402, t2403, t2404, t2408, t2411, t2416, t2430, t2569, t2614, t2617, t2832, t765, t775, t892);
    (t2798, t2801, t2802, t2804, t2806, t2810, t2811, t2815, t2828, t2829, t2832, t2836)
}
