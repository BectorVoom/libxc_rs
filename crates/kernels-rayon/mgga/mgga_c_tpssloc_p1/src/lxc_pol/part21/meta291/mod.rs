//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta291 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1602;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1603;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1604;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1605;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta291(t10544: f64, t276: f64, t285: f64, t273: f64, t2897: f64, t300: f64, t2928: f64, t941: f64, t2931: f64, t323: f64, t2784: f64, t892: f64, t2841: f64, t888: f64, t2840: f64, t287: f64, t275: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10577, t10595, t10599, t10608, t10623, t10629) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1602(t10544, t276, t285, t273, t2897, t300, t2928, t941);
        let t10632 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1603(t2931, t323);
        let (t10636, t10650, t10655) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1604(t10544, t2784, t892, t2841, t888);
        let (t10660, t10661) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1605(t2840, t287, t275);
    (t10577, t10595, t10599, t10608, t10623, t10629, t10632, t10636, t10650, t10655, t10660, t10661)
}
