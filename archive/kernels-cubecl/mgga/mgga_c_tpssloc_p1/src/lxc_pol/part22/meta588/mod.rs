//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta588 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2100;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2101;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta588<F: Float>(t13123: F, t9467: F, t4199: F, t9713: F, t1471: F, t31: F, t4211: F, t9874: F, t13119: F, t2663: F, t2517: F, t4098: F, t1472: F, t9862: F, t32: F, t4094: F, t10109: F, t1527: F, t1496: F, t41083: F, t4257: F, t9601: F, t4261: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t46371, t46376, t46387, t46433, t46436, t46437) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2100::<F>(t13123, t9467, t4199, t9713, t1471, t31, t4211, t9874, t13119, t2663, t2517, t4098);
        let (t46438, t46439, t46447, t46488, t46546, t46550, t46573) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2101::<F>(t46437, t1472, t9862, t32, t4094, t10109, t1527, t1496, t41083, t4257, t9601, t4261);
    (t46371, t46376, t46387, t46433, t46436, t46438, t46439, t46447, t46488, t46546, t46550, t46573)
}
