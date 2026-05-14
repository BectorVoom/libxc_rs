//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 898/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk898<F: Float>(t11470: F, t354: F, t2867: F, t481: F, t3574: F, t792: F, t2333: F, t910: F, t795: F, t105: F, t920: F, t97: F, t3582: F, t106: F, t2530: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t11471 = t354 * t11470;
    let t11475 = t2867 * t481;
    let t11486 = t3574 * t792;
    let t11496 = t2333 * t910;
    let t11497 = t11496 * t795;
    let t11505 = t105 * t920;
    let t11506 = t97 * t11505;
    let t11509 = t3574 * t481;
    let t11518 = t3582 * t481;
    let t11523 = t97 * t106 * t2530;
    (t11471, t11475, t11486, t11496, t11497, t11505, t11506, t11509, t11518, t11523)
}
