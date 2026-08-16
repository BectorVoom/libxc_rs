//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta98 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk548;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk549;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk550;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta98<F: Float>(t2771: F, t2826: F, t136: F, t2776: F, t908: F, t2780: F, t2766: F, t2773: F, t2778: F, t2782: F, t2800: F, t2808: F, t2810: F, t2816: F, t2818: F, t2823: F, t2824: F, t913: F, t893: F, t891: F, t275: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t2827, t2828, t2830, t2831, t2833, t2834, t2836) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk548::<F>(t2771, t2826, t136, t2776, t908, t2780, t2766, t2773, t2778, t2782, t2800, t2808, t2810, t2816, t2818, t2823, t2824);
        let (t2837, t2839, t2840) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk549::<F>(t2836, t913, t893, t891);
        let (t2841, t2842) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk550::<F>(t2840, t275);
    (t2827, t2828, t2830, t2831, t2833, t2834, t2836, t2837, t2839, t2840, t2841, t2842)
}
