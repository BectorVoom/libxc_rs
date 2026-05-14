//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 990/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk990<F: Float>(t1511: F, t5152: F, t1545: F, t1628: F, t1548: F, t16540: F, t4920: F, t541: F, t555: F, t1527: F, t1598: F, t1601: F, t479: F, t490: F, t1662: F, t1542: F) -> (F, F, F, F, F, F, F) {
    let t16617 = t1511 * t5152;
    let t16619 = t1545 * t1628;
    let t16621 = t1548 * t1628;
    let t16626 = 0.14035736694323150897e2 * t555 * t4920 * t16540 * t541;
    let t16631 = 0.34367190188705947437e1 * t479 * t1598 * t1527 * t1601 * t490;
    let t16632 = t1545 * t1662;
    let t16638 = t1542 * t1628;
    (t16617, t16619, t16621, t16626, t16631, t16632, t16638)
}
