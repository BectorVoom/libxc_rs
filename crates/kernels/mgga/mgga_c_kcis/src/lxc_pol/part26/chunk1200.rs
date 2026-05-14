//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1200/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1200<F: Float>(t8196: F, t97772: F, t22630: F, t573: F, t28589: F, t28597: F, t1552: F, t20961: F, t22212: F, t6028: F, t7948: F, t8191: F, t97727: F, t22699: F, t491: F, t7949: F) -> (F, F, F, F, F, F, F) {
    let t103028 = t97772 * t8196;
    let t103031 = t22630 * t573;
    let t103033 = t28589 * t28597;
    let t103035 = t20961 * t1552;
    let t103038 = t7948 * t6028 * t22212;
    let t103040 = t97727 * t8191;
    let t103042 = t22699 * t491;
    let t103043 = t103042 * t7949;
    (t103028, t103031, t103033, t103035, t103038, t103040, t103043)
}
