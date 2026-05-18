//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1128/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1128<F: Float>(t16490: F, t2182: F, t16500: F, t2164: F, t16390: F, t9896: F, t16398: F, t7110: F, t16386: F, t16394: F, t16554: F, t7122: F) -> (F, F, F, F, F, F, F) {
    let t48748 = t2182 * t16490;
    let t48750 = t2164 * t16500;
    let t48806 = t9896 * t16390;
    let t48808 = t7110 * t16398;
    let t48810 = t9896 * t16386;
    let t48812 = t7110 * t16394;
    let t48862 = t7122 * t16554;
    (t48748, t48750, t48806, t48808, t48810, t48812, t48862)
}
