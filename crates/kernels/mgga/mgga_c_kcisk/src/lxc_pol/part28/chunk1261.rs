//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1261/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1261<F: Float>(t1136: F, t15722: F, t4825: F, t11195: F, t604: F, t670: F, t11196: F, t1689: F, t11700: F, t1904: F, t11699: F, t724: F, t751: F, t5438: F, t11984: F, t772: F, t794: F) -> (F, F, F, F, F, F, F, F) {
    let t44181 = t1136 * t15722;
    let t44406 = t4825 * t4825;
    let t44407 = 1.0 / t44406;
    let t46460 = t604 / t11195 / t670;
    let t46928 = t1689 * t11196;
    let t47024 = t1904 * t11700;
    let t47033 = t724 / t11699 / t751;
    let t47648 = t5438 * t5438;
    let t47649 = 1.0 / t47648;
    let t48363 = t772 / t11984 / t794;
    (t44181, t44407, t46460, t46928, t47024, t47033, t47649, t48363)
}
