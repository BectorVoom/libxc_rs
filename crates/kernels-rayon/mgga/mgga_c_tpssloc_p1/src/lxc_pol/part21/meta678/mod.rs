//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta678 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2485;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2486;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta678(t12858: f64, t2535: f64, t12606: f64, t707: f64, t751: f64, t4205: f64, t9868: f64, t193: f64, t776: f64, t3966: f64, t4194: f64, t607: f64, t750: f64, t1409: f64, t9862: f64, t13123: f64, t9467: f64, t4199: f64, t9713: f64, t1471: f64, t31: f64, t4211: f64, t9874: f64, t13119: f64, t2663: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t46310, t46317, t46335, t46341, t46348) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2485(t12858, t2535, t12606, t707, t751, t4205, t9868, t193, t776, t3966, t4194, t607, t750);
        let (t46369, t46371, t46376, t46387, t46433, t46435) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2486(t1409, t707, t9862, t13123, t9467, t4199, t9713, t1471, t31, t4211, t9874, t13119, t2663);
    (t46310, t46317, t46335, t46341, t46348, t46369, t46371, t46376, t46387, t46433, t46435)
}
