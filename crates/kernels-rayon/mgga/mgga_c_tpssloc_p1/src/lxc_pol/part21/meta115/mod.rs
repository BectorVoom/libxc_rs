//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta115 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk790;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk791;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk792;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk793;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta115(t2771: f64, t2826: f64, t136: f64, t2776: f64, t908: f64, t2780: f64, t2766: f64, t2773: f64, t2778: f64, t2782: f64, t2800: f64, t2808: f64, t2810: f64, t2816: f64, t2818: f64, t2823: f64, t2824: f64, t913: f64, t893: f64, t891: f64, t275: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2827, t2828, t2830, t2831, t2833, t2834, t2836) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk790(t2771, t2826, t136, t2776, t908, t2780, t2766, t2773, t2778, t2782, t2800, t2808, t2810, t2816, t2818, t2823, t2824);
        let (t2837, t2839, t2840) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk791(t2836, t913, t893, t891);
        let t2841 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk792(t2840);
        let t2842 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk793(t275, t2841);
    (t2827, t2828, t2830, t2831, t2833, t2834, t2836, t2837, t2839, t2840, t2841, t2842)
}
