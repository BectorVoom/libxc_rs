//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta167 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk890;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk891;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk892;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk893;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk894;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk895;
use chunk6::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk896;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta167<F: Float>(t187: F, t9363: F, t2490: F, t737: F, t2492: F, t744: F, t185: F, t2494: F, t1340: F, t2516: F, t4038: F, t9283: F, t9286: F, t9289: F, t9292: F, t9296: F, t9298: F, t9300: F, t9303: F, t738: F, t745: F, t1320: F, t3853: F, t123: F, t147: F, t9291: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t9365, t9367) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk890::<F>(t187, t9363, t2490, t737);
        let t9368 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk891::<F>(t2492, t744);
        let t9371 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk892::<F>(t185, t2494);
        let t9372 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk893::<F>(t9367, t9368, t9371);
        let (t9374, t9376, t9385) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk894::<F>(t1340, t9372, t2516, t4038, t9283, t9286, t9289, t9292, t9296, t9298, t9300, t9303);
        let t9387 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk895::<F>(t738, t745, t9385);
        let (t9389, t9391, t9394) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk896::<F>(t1340, t9387, t1320, t3853, t123, t147, t9291);
    (t9365, t9367, t9368, t9371, t9372, t9374, t9376, t9385, t9387, t9389, t9391, t9394)
}
