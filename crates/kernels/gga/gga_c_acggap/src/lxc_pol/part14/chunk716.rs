//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 716/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk716<F: Float>(t1181: F, t8648: F, t7493: F, t1427: F, t599: F, t8463: F, t1165: F, t1432: F, t7351: F, t7426: F, t1439: F, t7575: F, t1992: F, t525: F, t7842: F, t7585: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t8649 = t1181 * t8648;
    let t8650 = t7493 * t8649;
    let t8652 = t599 * t1427;
    let t8653 = t1181 * t8652;
    let t8654 = t8463 * t8653;
    let t8657 = t1165 * t7351 * t1432;
    let t8658 = t7426 * t8657;
    let t8661 = t1165 * t7351 * t1439;
    let t8662 = t7575 * t8661;
    let t8665 = t7842 * t1992 * t525;
    let t8666 = t7585 * t8665;
    (t8649, t8650, t8652, t8653, t8654, t8657, t8658, t8661, t8662, t8665, t8666)
}
