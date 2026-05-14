//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 942/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk942<F: Float>(t11180: F, t2320: F, t1174: F, t3743: F, t6149: F, t3041: F, t3747: F, t11155: F, t6156: F, t7955: F, t9782: F, t834: F, t6165: F, t3052: F, t841: F, t1167: F, t3730: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t11181 = t11180 * t2320;
    let t11184 = t3743 * t1174;
    let t11185 = t6149 * t11184;
    let t11187 = t3041 * t3747;
    let t11190 = -t6156 + 4.0 / 3.0 * t7955 - t9782 + t11155;
    let t11191 = t834 * t11190;
    let t11196 = t6165 * t11184;
    let t11198 = t3052 * t3747;
    let t11200 = t841 * t11190;
    let t11205 = t1167 * t3730;
    (t11181, t11184, t11185, t11187, t11190, t11191, t11196, t11198, t11200, t11205)
}
