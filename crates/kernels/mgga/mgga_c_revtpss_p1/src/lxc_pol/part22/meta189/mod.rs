//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta189 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1205;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1206;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1207;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1208;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1209;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1210;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta189<F: Float>(t2465: F, t4481: F, t1579: F, t886: F, t2770: F, t1558: F, t251: F, t231: F, t2783: F, t2782: F, t1559: F, t72: F, t686: F, t2798: F, t225: F, t2718: F, t213: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t4482, t4487, t4494) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1205::<F>(t2465, t4481, t1579, t886, t2770, t1558, t251);
        let t4496 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1206::<F>(t231, t2783, t4494);
        let (t4497, t4499, t4500) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1207::<F>(t2782, t4496, t1559, t72, t686);
        let (t4501, t4503) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1208::<F>(t2798, t4500, t225, t2718);
        let t4504 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1209::<F>(t213, t4503);
        let t4514 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1210::<F>(t213, t2783);
    (t4482, t4487, t4494, t4496, t4497, t4499, t4500, t4501, t4503, t4504, t4514)
}
