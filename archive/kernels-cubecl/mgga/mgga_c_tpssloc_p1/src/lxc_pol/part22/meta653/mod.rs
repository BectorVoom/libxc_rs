//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta653 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2195;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta653<F: Float>(t16898: F, t9638: F, t13258: F, t16893: F, t16918: F, t4191: F, t46657: F, t4240: F, t120: F, t16752: F, t16924: F, t17004: F, t2563: F) -> (F, F, F, F, F, F, F, F) {
        let (t58461, t58472, t58474, t58480, t58482, t58495, t58504, t58528) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2195::<F>(t16898, t9638, t13258, t16893, t16918, t4191, t46657, t4240, t120, t16752, t16924, t17004, t2563);
    (t58461, t58472, t58474, t58480, t58482, t58495, t58504, t58528)
}
