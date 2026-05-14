//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 909/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk909<F: Float>(t11004: F, t3579: F, t3582: F, t792: F, t10997: F, t3275: F, t6967: F, t795: F, t3263: F, t1561: F, t3617: F, t3277: F, t10918: F, t2867: F, t11479: F, t3262: F, t3264: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t11618 = t3579 * t11004;
    let t11619 = 5.0 / 16.0 * t11618;
    let t11621 = t3582 * t792;
    let t11622 = t10997 * t11621;
    let t11623 = t3275 * t11622;
    let t11624 = 45.0 / 64.0 * t11623;
    let t11625 = t6967 * t795;
    let t11626 = t3263 * t11625;
    let t11627 = t3275 * t11626;
    let t11628 = t11627 / 2.0;
    let t11629 = t1561 * t3617;
    let t11631 = t3275 * t11629 * t3277;
    let t11632 = 5.0 / 16.0 * t11631;
    let t11634 = t3275 * t10918 * t2867;
    let t11635 = t11634 / 4.0;
    let t11637 = t3262 * t11479 * t3264;
    (t11618, t11619, t11621, t11622, t11623, t11624, t11625, t11626, t11627, t11628, t11629, t11631, t11632, t11634, t11635, t11637)
}
