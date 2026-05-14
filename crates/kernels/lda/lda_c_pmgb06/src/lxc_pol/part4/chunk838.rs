//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 838/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk838<F: Float>(t2653: F, t489: F, t161: F, t2630: F, t435: F, t132: F, t2624: F, t2018: F, t831: F, t2649: F, t2015: F, t802: F, t2605: F, t337: F, t6560: F, t5069: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t6610 = t489 * t2653;
    let t6611 = t161 * t6610;
    let t6612 = 2.0 / 45.0 * t6611;
    let t6613 = t435 * t2630;
    let t6614 = t132 * t6613;
    let t6615 = 2.0 / 45.0 * t6614;
    let t6616 = t489 * t2624;
    let t6617 = t161 * t6616;
    let t6618 = t6617 / 45.0;
    let t6619 = t831 * t2018;
    let t6620 = 2.0 / 45.0 * t6619;
    let t6621 = t435 * t2649;
    let t6622 = t132 * t6621;
    let t6623 = t6622 / 45.0;
    let t6624 = t802 * t2015;
    let t6625 = 2.0 / 45.0 * t6624;
    let t6626 = t435 * t2605;
    let t6627 = t132 * t6626;
    let t6628 = 2.0 / 45.0 * t6627;
    let t6629 = t6560 * t337;
    let t6630 = t5069 * t6629;
    (t6610, t6612, t6613, t6615, t6616, t6618, t6620, t6621, t6623, t6625, t6626, t6628, t6629, t6630)
}
