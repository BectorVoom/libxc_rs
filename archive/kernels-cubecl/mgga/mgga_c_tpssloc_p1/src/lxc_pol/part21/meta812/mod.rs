//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta812 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2850;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2851;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2852;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2853;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2854;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2855;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2856;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta812<F: Float>(t10277: F, t2244: F, t5398: F, t123: F, t2768: F, t17177: F, t882: F, t41687: F, t5392: F, t10564: F, t17151: F, t17168: F, t690: F, t17172: F, t2250: F, t5677: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t59742, t59744) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2850::<F>(t10277, t2244, t5398, t123, t2768);
        let (t59746, t59748) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2851::<F>(t17177, t2244, t123, t882);
        let (t59751, t59753) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2852::<F>(t2244, t41687, t5392, t10564, t123);
        let (t59755, t59757) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2853::<F>(t17151, t2244, t123, t2768);
        let t59759 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2854::<F>(t17168, t690);
        let t59761 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2855::<F>(t17172, t690);
        let (t59763, t59765) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2856::<F>(t2250, t5677, t123, t882);
    (t59742, t59744, t59746, t59748, t59751, t59753, t59755, t59757, t59759, t59761, t59763, t59765)
}
