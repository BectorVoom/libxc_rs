//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta101 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk559;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk560;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk561;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk562;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk563;
use chunk5::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk564;
use chunk6::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk565;
use chunk7::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk566;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta101<F: Float>(t2904: F, t315: F, t950: F, t951: F, t2764: F, t2822: F, t2766: F, t2773: F, t2778: F, t2782: F, t2800: F, t2808: F, t2816: F, t2818: F, t2824: F, t2828: F, t2831: F, t2834: F, t941: F, t323: F, t2786: F, t2789: F, t2796: F, t2839: F, t2847: F, t2853: F, t2856: F, t2861: F, t2863: F, t2881: F, t2886: F, t2889: F, t2898: F, t2900: F, t311: F, t924: F, t933: F, t943: F, t952: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t2905, t2906) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk559::<F>(t2904, t315, t950);
        let (t2907, t2924) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk560::<F>(t2906, t951, t2764, t2822, t2766, t2773, t2778, t2782, t2800, t2808, t2816, t2818, t2824, t2828, t2831, t2834);
        let t2925 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk561::<F>(t2924, t951);
        let t2928 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk562::<F>(t941);
        let t2929 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk563::<F>(t2928);
        let t2930 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk564::<F>(t2929, t315);
        let (t2931, t2932) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk565::<F>(t323);
        let (t2933, t2936) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk566::<F>(t2906, t2932, t2786, t2789, t2796, t2839, t2847, t2853, t2856, t2861, t2863, t2881, t2886, t2889, t2898, t2900, t2905, t2907, t2925, t2930, t311, t924, t933, t943, t952);
    (t2905, t2906, t2907, t2924, t2925, t2928, t2929, t2930, t2931, t2932, t2933, t2936)
}
