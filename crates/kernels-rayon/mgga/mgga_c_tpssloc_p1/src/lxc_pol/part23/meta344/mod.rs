//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta344 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1130;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1131;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1132;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta344(t2225: f64, t3824: f64, t1287: f64, t9214: f64, t39033: f64, t522: f64, t39035: f64, t39031: f64, t16: f64, t185: f64, t520: f64, t9212: f64, t9218: f64, t118: f64, t142: f64, t39283: f64, t2223: f64, t2475: f64, t2461: f64, t2478: f64, t159: f64, t172: f64, t2454: f64, t268: f64, t39249: f64, t39256: f64, t39300: f64, t39309: f64, t39312: f64, t39316: f64, t39320: f64, t39377: f64, t39378: f64, t39381: f64, t39535: f64, t676: f64, t724: f64, t732: f64, t739: f64, t740: f64, t746: f64, t747: f64, t781: f64, t9493: f64, t9720: f64, t9738: f64, t9740: f64, t9752: f64, t9762: f64, t9763: f64, t9781: f64, t9828: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t39595, t39597, t39604, t39606, t39608, t39615, t39634) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1130(t2225, t3824, t1287, t9214, t39033, t522, t39035, t39031, t16, t185, t520, t9212);
        let (t39635, t39655, t39658) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1131(t39634, t1287, t9218, t118, t142, t39283);
        let (t39660, t39664, t39706) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1132(t2223, t3824, t2475, t2461, t2478, t159, t172, t2454, t268, t39249, t39256, t39300, t39309, t39312, t39316, t39320, t39377, t39378, t39381, t39535, t676, t724, t732, t739, t740, t746, t747, t781, t9493, t9720, t9738, t9740, t9752, t9762, t9763, t9781, t9828);
    (t39595, t39597, t39604, t39606, t39608, t39615, t39635, t39655, t39658, t39660, t39664, t39706)
}
