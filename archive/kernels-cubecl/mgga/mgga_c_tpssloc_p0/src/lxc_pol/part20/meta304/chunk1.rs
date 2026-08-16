//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1548/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1548<F: Float>(t11205: F, t136: F, t1113: F, t11172: F, t1114: F, t2403: F) -> (F, F, F, F) {
    let t11206 = t136 * t11205;
    let t11208 = t1113 * t11172;
    let t11209 = t136 * t11208;
    let t11211 = t2403 * t1114;
    (t11206, t11208, t11209, t11211)
}
