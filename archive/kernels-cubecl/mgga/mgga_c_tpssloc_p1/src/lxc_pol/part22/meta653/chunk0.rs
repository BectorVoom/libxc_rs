//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2195/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2195<F: Float>(t16898: F, t9638: F, t13258: F, t16893: F, t16918: F, t4191: F, t46657: F, t4240: F, t120: F, t16752: F, t16924: F, t17004: F, t2563: F) -> (F, F, F, F, F, F, F, F) {
    let t58461 = t9638 * t16898;
    let t58472 = t13258 * t16893;
    let t58474 = t9638 * t16918;
    let t58480 = t46657 * t4191;
    let t58482 = t46657 * t4240;
    let t58495 = t120 * t16752;
    let t58504 = t9638 * t16924;
    let t58528 = t2563 * t17004;
    (t58461, t58472, t58474, t58480, t58482, t58495, t58504, t58528)
}
