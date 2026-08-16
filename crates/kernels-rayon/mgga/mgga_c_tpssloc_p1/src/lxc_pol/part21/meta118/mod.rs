//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta118 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk804;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk805;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk806;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk807;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk808;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk809;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta118(t2862: f64, t2888: f64, t2764: f64, t2766: f64, t2773: f64, t2778: f64, t2782: f64, t324: f64, t938: f64, t942: f64, t320: f64, t941: f64, t315: f64, t950: f64, t951: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2889, t2892, t2897) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk804(t2862, t2888, t2764, t2766, t2773, t2778, t2782);
        let (t2898, t2900) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk805(t2897, t324, t938, t942);
        let (t2903, t2904) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk806(t320, t941);
        let t2905 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk807(t2904, t315);
        let t2906 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk808(t950);
        let t2907 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk809(t2906, t951);
    (t2889, t2892, t2897, t2898, t2900, t2903, t2904, t2905, t2906, t2907)
}
