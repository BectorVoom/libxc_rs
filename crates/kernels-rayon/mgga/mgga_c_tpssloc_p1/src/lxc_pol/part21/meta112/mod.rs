//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta112 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk774;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk775;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk776;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk777;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk778;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk779;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk780;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta112(t2768: f64, t2771: f64, t123: f64, t2289: f64, t2244: f64, t882: f64, t2250: f64, t883: f64, t2765: f64, t2766: f64, t291: f64, t888: f64, t892: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2772, t2773) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk774(t2768, t2771, t123);
        let t2775 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk775(t2289);
        let t2776 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk776(t2244, t2775);
        let (t2777, t2778) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk777(t2776, t882, t123);
        let t2780 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk778(t2250, t883);
        let (t2781, t2782) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk779(t2780, t882, t123);
        let (t2784, t2786, t2787) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk780(t2765, t2766, t2773, t2778, t2782, t291, t888, t892);
    (t2772, t2773, t2775, t2776, t2777, t2778, t2780, t2781, t2782, t2784, t2786, t2787)
}
