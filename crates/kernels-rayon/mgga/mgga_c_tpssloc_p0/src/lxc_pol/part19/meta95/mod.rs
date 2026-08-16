//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta95 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk540;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk541;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk542;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk543;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta95(t2250: f64, t883: f64, t882: f64, t123: f64, t2765: f64, t2766: f64, t2773: f64, t2778: f64, t291: f64, t888: f64, t892: f64, t914: f64, t287: f64, t891: f64, t275: f64, t912: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t2780 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk540(t2250, t883);
        let (t2781, t2782) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk541(t2780, t882, t123);
        let (t2784, t2786, t2787, t2789, t2790, t2791, t2792) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk542(t2765, t2766, t2773, t2778, t2782, t291, t888, t892, t914, t287, t891, t275);
        let t2793 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk543(t912);
    (t2780, t2781, t2782, t2784, t2786, t2787, t2789, t2790, t2791, t2792, t2793)
}
