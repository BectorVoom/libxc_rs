//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta619 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1869;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1870;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta619(t1307: f64, t26331: f64, t26446: f64, t96951: f64, t1992: f64, t550: f64, t57545: f64, t6976: f64, t19893: f64, t90914: f64, t90915: f64, t1799: f64, t1834: f64, t1352: f64, t22633: f64, t19743: f64, t3807: f64, t20014: f64, t1351: f64, t6434: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t96954, t96958, t96962, t96964) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1869(t1307, t26331, t26446, t96951, t1992, t550, t57545, t6976, t19893, t90914, t90915, t1799, t1834);
        let (t96967, t96972, t96976, t96979, t96986) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1870(t1352, t22633, t6976, t96964, t96951, t19743, t3807, t1992, t20014, t1351, t550, t6434);
    (t96954, t96958, t96962, t96964, t96967, t96972, t96976, t96979, t96986)
}
