//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 863/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk863<F: Float>(t6121: F, t6133: F, t360: F, t2149: F, t6127: F, t1582: F, t259: F) -> (F, F, F, F, F) {
    let t6140 = t6133 * t6121;
    let t6141 = t360 * t6140;
    let t6144 = t2149 * t6127;
    let t6145 = t360 * t6144;
    let t6148 = t1582 * t259;
    (t6140, t6141, t6144, t6145, t6148)
}
