//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1228/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1228<F: Float>(t10525: F, t283: F, t2865: F, t374: F, t1165: F, t982: F, t1169: F, t3473: F, t3463: F, t3329: F, t7738: F, t3668: F, t7807: F) -> (F, F, F, F, F, F, F, F) {
    let t92522 = t10525 * t283;
    let t92525 = t374 * t2865;
    let t92532 = t1165 * t982;
    let t92537 = t1169 * t2865;
    let t92540 = t3473 * t982;
    let t92544 = t3463 * t982;
    let t92564 = t7738 * t3329;
    let t92576 = t7807 * t3668;
    (t92522, t92525, t92532, t92537, t92540, t92544, t92564, t92576)
}
