//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta86 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk497;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk498;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta86(t287: f64, t891: f64, t275: f64, t273: f64, t276: f64, t2764: f64, t241: f64, t63: f64, t281: f64, t283: f64, t976: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2790, t2791, t2792, t2798, t2802, t2810, t2815, t2820, t2822, t2823, t2826) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk497(t287, t891, t275, t273, t276, t2764, t241, t63, t281, t283, t976);
        let t2840 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk498(t891);
    (t2790, t2791, t2792, t2798, t2802, t2810, t2815, t2820, t2822, t2823, t2826, t2840)
}
