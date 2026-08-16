//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta116 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk754;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk755;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk756;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk757;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk758;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk759;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta116<F: Float>(t2846: F, t221: F, t346: F, t696: F, t345: F, t1003: F, t1007: F, t360: F, t365: F, t1038: F, t72: F, t1087: F, t1066: F, t828: F, t1043: F, t73: F, t357: F, t905: F, t606: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t3070, t3080, t3082, t3086, t3088) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk754::<F>(t2846, t221, t346, t696, t345, t1003, t1007, t360, t365);
        let t3089 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk755::<F>(t1038, t72);
        let t3090 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk756::<F>(t3088, t3089);
        let t3091 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk757::<F>(t1087, t3090);
        let t3092 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk758::<F>(t1066, t828);
        let (t3093, t3094, t3095) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk759::<F>(t1043, t73, t357, t905, t606);
    (t3070, t3080, t3082, t3086, t3088, t3089, t3090, t3091, t3092, t3093, t3094, t3095)
}
