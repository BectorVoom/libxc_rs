//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta17 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk133;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk134;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk135;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk136;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk137;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk138;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta17<F: Float>(t273: F, t276: F, t279: F, t285: F, t315: F, t293: F, t300: F, t302: F, t311: F, t199: F, t240: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t320, t323, t324) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk133::<F>(t273, t276, t279, t285);
        let (t328, t330, t334, t335) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk134::<F>(t315, t324, t293, t300, t302, t311, t199, t240, zeta_threshold);
        let t336 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk135::<F>(t334, t335);
        let t338 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk136::<F>(t273);
        let (t340, t341) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk137::<F>(t273);
        let t342 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk138::<F>(t338, t341);
    (t320, t323, t324, t328, t330, t334, t335, t336, t338, t340, t341, t342)
}
