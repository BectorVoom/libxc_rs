//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1016/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1016<F: Float>(t11184: F, t6165: F, t3052: F, t3747: F, t11190: F, t841: F, t1167: F, t3730: F, t218: F, t219: F, t11153: F, t334: F) -> (F, F, F, F, F, F) {
    let t11196 = t6165 * t11184;
    let t11198 = t3052 * t3747;
    let t11200 = t841 * t11190;
    let t11205 = t1167 * t3730;
    let t11207 = t218 * t219 * t11205;
    let t11209 = t334 * t11153;
    (t11196, t11198, t11200, t11205, t11207, t11209)
}
