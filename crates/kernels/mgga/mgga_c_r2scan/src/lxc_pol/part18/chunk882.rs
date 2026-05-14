//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 882/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk882<F: Float>(t11004: F, t3579: F, t3582: F, t792: F, t10997: F, t3275: F, t6967: F, t795: F, t3263: F, t1561: F, t3617: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
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
    (t11618, t11619, t11621, t11622, t11623, t11624, t11625, t11626, t11627, t11628, t11629)
}
