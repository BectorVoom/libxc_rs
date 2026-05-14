//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 694/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk694<F: Float>(t11490: F, t11645: F, t673: F, t716: F, t720: F, t415: F, t1333: F, t5177: F, t1871: F, t5174: F, t1895: F, t1869: F, t4811: F, t4818: F, t4817: F, t5069: F, sigma2: F) -> (F, F, F, F, F, F, F) {
    let t11646 = t11490 + t11645;
    let t11647 = t673 * t11646;
    let t11648 = t11647 * t716;
    let t11649 = t11648 * t720;
    let t11650 = t415 * t11649;
    let t11652 = t1333 * t5177;
    let t11658 = t5174 * t1871;
    let t11659 = t11658 * sigma2;
    let t11660 = t11659 * t1895;
    let t11661 = t1869 * t11660;
    let t11663 = t4811 * t4818;
    let t11668 = t4817 * t5069;
    (t11646, t11650, t11652, t11658, t11661, t11663, t11668)
}
