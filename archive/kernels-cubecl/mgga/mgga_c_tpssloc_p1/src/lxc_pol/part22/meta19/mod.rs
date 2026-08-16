//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta19 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk147;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk148;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk149;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk150;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk151;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk152;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk153;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta19<F: Float>(t360: F, sigma0: F, t34: F, t35: F, rho0: F, t354: F, t335: F, t67: F, t246: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t361, t362, t363) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk147::<F>(t360, sigma0);
        let (t364, t365, t368) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk148::<F>(t362, t363, t34, t35, rho0);
        let t369 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk149::<F>(t364, t368);
        let t370 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk150::<F>(t354, t369);
        let t371 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk151::<F>(t335);
        let t372 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk152::<F>(t371);
        let (t373, t374) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk153::<F>(t372, t67, t246);
    (t361, t362, t363, t364, t365, t368, t369, t370, t371, t372, t373, t374)
}
