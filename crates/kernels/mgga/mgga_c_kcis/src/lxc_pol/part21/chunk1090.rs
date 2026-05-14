//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1090/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1090<F: Float>(t1169: F, t2865: F, t3473: F, t982: F, t3463: F, t3329: F, t7738: F, t3668: F, t7807: F, t10497: F, t2183: F, t11068: F, t27002: F, t7788: F, t11178: F, t1250: F, t251: F) -> (F, F, F, F, F, F, F, F) {
    let t92537 = t1169 * t2865;
    let t92540 = t3473 * t982;
    let t92544 = t3463 * t982;
    let t92564 = t7738 * t3329;
    let t92576 = t7807 * t3668;
    let t92581 = t2183 * t10497;
    let t92587 = t7788 * t11068 * t27002;
    let t92590 = t11178 * t251 * t1250;
    (t92537, t92540, t92544, t92564, t92576, t92581, t92587, t92590)
}
