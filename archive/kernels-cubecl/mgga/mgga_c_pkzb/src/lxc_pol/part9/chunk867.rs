//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 867/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk867<F: Float>(t2317: F, t877: F, t2249: F, t862: F, t2278: F, t858: F) -> (F, F, F) {
    let t6300 = t877 * t2317;
    let t6303 = t2249 * t862;
    let t6308 = t858 * t2278;
    (t6300, t6303, t6308)
}
