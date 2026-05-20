//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta187 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1198;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1199;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1200;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta187<F: Float>(t4424: F, t827: F, t828: F, t1559: F, t221: F, t2485: F, t2484: F, t1544: F, t775: F, t2477: F, t2672: F, t2686: F, t2704: F, t2742: F, t4345: F, t4350: F, t4355: F, t4357: F, t4359: F, t4362: F, t4368: F, t4373: F, t825: F, t851: F, t1548: F, t800: F, t4365: F, t837: F, t4364: F, t125: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t4426, t4430, t4431, t4433) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1198::<F>(t4424, t827, t828, t1559, t221, t2485, t2484, t1544, t775);
        let (t4435, t4439) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1199::<F>(t2477, t4433, t828, t2672, t2686, t2704, t2742, t4345, t4350, t4355, t4357, t4359, t4362, t4368, t4373, t4426, t4431, t825, t851);
        let (t4442, t4447, t4450) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1200::<F>(t1548, t775, t800, t4365, t837, t4364, t125, t1544);
    (t4426, t4430, t4431, t4433, t4435, t4439, t4442, t4447, t4450)
}
