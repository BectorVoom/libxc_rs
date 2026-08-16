//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta119 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk810;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk811;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk812;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk813;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk814;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk815;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk816;
use chunk7::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk817;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta119(t2764: f64, t2822: f64, t2766: f64, t2773: f64, t2778: f64, t2782: f64, t2800: f64, t2808: f64, t2816: f64, t2818: f64, t2824: f64, t2828: f64, t2831: f64, t2834: f64, t951: f64, t941: f64, t315: f64, t323: f64, t2906: f64, t2786: f64, t2789: f64, t2796: f64, t2839: f64, t2847: f64, t2853: f64, t2856: f64, t2861: f64, t2863: f64, t2881: f64, t2886: f64, t2889: f64, t2898: f64, t2900: f64, t2905: f64, t2907: f64, t311: f64, t924: f64, t933: f64, t943: f64, t952: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2912, t2919, t2924) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk810(t2764, t2822, t2766, t2773, t2778, t2782, t2800, t2808, t2816, t2818, t2824, t2828, t2831, t2834);
        let t2925 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk811(t2924, t951);
        let t2928 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk812(t941);
        let t2929 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk813(t2928);
        let t2930 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk814(t2929, t315);
        let (t2931, t2932) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk815(t323);
        let t2933 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk816(t2906, t2932);
        let t2936 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk817(t2786, t2789, t2796, t2839, t2847, t2853, t2856, t2861, t2863, t2881, t2886, t2889, t2898, t2900, t2905, t2907, t2925, t2930, t2933, t311, t924, t933, t943, t952);
    (t2912, t2919, t2924, t2925, t2928, t2929, t2930, t2931, t2932, t2933, t2936)
}
