//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta249 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1047;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1048;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta249(t252: f64, t828: f64, t232: f64, t6646: f64, t1888: f64, t1894: f64, t852: f64, t214: f64, t1880: f64, t25: f64, t868: f64, t343: f64, t984: f64, t3034: f64, t334: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t6648, t6649, t6650, t6652, t6653, t6654, t6671, t6733) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1047(t252, t828, t232, t6646, t1888, t1894, t852, t214, t1880, t25, t868, t343, t984);
        let t6739 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1048(t3034, t334);
    (t6648, t6649, t6650, t6652, t6653, t6654, t6671, t6733, t6739)
}
