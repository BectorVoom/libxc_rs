//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta679 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2487;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2488;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta679<F: Float>(t2517: F, t4098: F, t1472: F, t9862: F, t13115: F, t9932: F, t32: F, t4094: F, t13034: F, t225: F, t10109: F, t1527: F, t13036: F, t13336: F, t68: F, t1496: F, t41083: F, t4257: F, t9601: F, t13193: F, t2697: F, t13204: F, t2563: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t46437, t46439, t46445, t46447, t46452, t46488) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2487::<F>(t2517, t4098, t1472, t9862, t13115, t9932, t32, t4094, t13034, t225, t10109, t1527);
        let (t46508, t46528, t46546, t46549, t46551, t46558) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2488::<F>(t13036, t225, t13336, t68, t1496, t41083, t4257, t9601, t13193, t2697, t13204, t2563);
    (t46437, t46439, t46445, t46447, t46452, t46488, t46508, t46528, t46546, t46549, t46551, t46558)
}
