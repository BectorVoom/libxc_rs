//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1253/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1253<F: Float>(t16617: F, t16619: F, t16621: F, t19754: F, t19756: F, t19758: F, t19775: F, t19778: F, t19795: F, t19797: F, t16632: F, t16634: F, t19803: F, t19805: F, t16638: F, t16626: F, t16631: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t24617 = 0.20508037716432813316e4 * t16617;
    let t24618 = 24.0 * t16619;
    let t24619 = 64.0 * t16621;
    let t24620 = 48.0 * t19754;
    let t24621 = 96.0 * t19756;
    let t24622 = 0.46785788981077169656e1 * t19758;
    let t24623 = 4.0 * t19775;
    let t24624 = 2.0 * t19778;
    let t24625 = 0.36622894612013090108e-3 * t19795;
    let t24626 = 64.0 * t19797;
    let t24627 = 12.0 * t16632;
    let t24628 = 32.0 * t16634;
    let t24629 = 160.0 * t19803;
    let t24630 = 240.0 * t19805;
    let t24631 = 120.0 * t16638;
    let t24632 = -t24617 - t24618 + t24619 - t24620 + t24621 + t24622 + t24623 + t24624 + t16626 - t16631 - t24625 - t24626 + t24627 + t24628 + t24629 - t24630 + t24631;
    (t24617, t24618, t24619, t24620, t24621, t24622, t24623, t24624, t24625, t24626, t24627, t24628, t24629, t24630, t24631, t24632)
}
