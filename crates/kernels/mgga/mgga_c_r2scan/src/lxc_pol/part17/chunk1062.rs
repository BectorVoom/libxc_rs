//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1062/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1062<F: Float>(t10977: F, t10981: F, t37364: F, t10950: F, t11015: F, t3434: F, t1654: F, t874: F, t122: F, t158: F, t166: F, t23: F, t23102: F, t261: F, t603: F, t784: F, t875: F) -> (F, F, F, F, F) {
    let t37480 = t37364 * t10977 * t10981;
    let t37483 = t3434 * t11015 * t10950;
    let t37501 = t1654 * t874;
    let t37505 = t1654 * t122;
    let t37523 = t23102 / t23 / t603 * t875 * t158 * t166 * t784 * t261;
    (t37480, t37483, t37501, t37505, t37523)
}
