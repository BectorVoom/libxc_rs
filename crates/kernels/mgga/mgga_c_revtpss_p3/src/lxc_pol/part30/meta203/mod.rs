//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta203 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk982;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk983;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk984;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk985;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta203<F: Float>(t4416: F, t775: F, t4343: F, t832: F, t1553: F, t1555: F, t227: F, t229: F, t4409: F, t4415: F, t830: F, t833: F, t231: F, t827: F, t828: F, t1559: F, t221: F, t2485: F, t2484: F, t1544: F, t2477: F, t2672: F, t2686: F, t2704: F, t2742: F, t4345: F, t4350: F, t4355: F, t4357: F, t4359: F, t4362: F, t4368: F, t4373: F, t825: F, t851: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t4417, t4420, t4423) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk982::<F>(t4416, t775, t4343, t832, t1553, t1555, t227, t229, t4409, t4415, t830, t833);
        let t4424 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk983::<F>(t231, t4423);
        let (t4426, t4430, t4431, t4433) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk984::<F>(t4424, t827, t828, t1559, t221, t2485, t2484, t1544, t775);
        let (t4435, t4439) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk985::<F>(t2477, t4433, t828, t2672, t2686, t2704, t2742, t4345, t4350, t4355, t4357, t4359, t4362, t4368, t4373, t4426, t4431, t825, t851);
    (t4417, t4420, t4423, t4424, t4426, t4430, t4433, t4435, t4439)
}
