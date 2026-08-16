//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2147/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2147<F: Float>(t19739: F, t22633: F, t3807: F, t6976: F, t28131: F, t81159: F, t552: F, t6434: F, t1307: F, t6637: F, t6888: F, t26331: F, t26446: F, t96964: F) -> (F, F, F, F) {
    let t97119 = t22633 * t6976 * t19739 * t3807;
    let t97124 = t81159 * t28131;
    let t97126 = t552 * t6434;
    let t97129 = t6888 * t6637 * t97126 * t1307;
    let t97135 = t26331 * t26446 * t96964 * t1307;
    (t97119, t97124, t97129, t97135)
}
