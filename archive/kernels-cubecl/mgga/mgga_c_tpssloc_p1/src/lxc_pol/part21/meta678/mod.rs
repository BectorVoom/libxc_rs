//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta678 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2485;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2486;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta678<F: Float>(t12858: F, t2535: F, t12606: F, t707: F, t751: F, t4205: F, t9868: F, t193: F, t776: F, t3966: F, t4194: F, t607: F, t750: F, t1409: F, t9862: F, t13123: F, t9467: F, t4199: F, t9713: F, t1471: F, t31: F, t4211: F, t9874: F, t13119: F, t2663: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t46310, t46317, t46335, t46341, t46348) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2485::<F>(t12858, t2535, t12606, t707, t751, t4205, t9868, t193, t776, t3966, t4194, t607, t750);
        let (t46369, t46371, t46376, t46387, t46433, t46435) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2486::<F>(t1409, t707, t9862, t13123, t9467, t4199, t9713, t1471, t31, t4211, t9874, t13119, t2663);
    (t46310, t46317, t46335, t46341, t46348, t46369, t46371, t46376, t46387, t46433, t46435)
}
