//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1149/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1149<F: Float>(t300: F, t3880: F, t931: F, t10107: F, t3174: F, t68: F, t10111: F, t3874: F, t19191: F, t2380: F, t3899: F, t10084: F, t3206: F, t926: F) -> (F, F, F, F, F, F) {
    let t27020 = t300 * t931 * t3880;
    let t27028 = t3174 * t68 * t10107;
    let t27031 = t3174 * t68 * t10111;
    let t27044 = t300 * t931 * t3874;
    let t27073 = t2380 * t19191 * t3899;
    let t27076 = t3206 * t926 * t10084;
    (t27020, t27028, t27031, t27044, t27073, t27076)
}
