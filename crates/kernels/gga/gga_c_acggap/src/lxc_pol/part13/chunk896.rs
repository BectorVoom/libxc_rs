//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 896/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk896<F: Float>(t30657: F, t1020: F, t7605: F, t1205: F, t7614: F, t30228: F, t601: F, t30174: F, t151: F, t56: F, t593: F, t606: F) -> (F, F, F, F, F, F) {
    let t30658 = F::cast_from(0.32155513588552302729e-3_f64) * t30657;
    let t30659 = t7605 * t1020;
    let t30661 = t7614 * t1205;
    let t30663 = t30228 * t601;
    let t30664 = F::cast_from(0.19293308153131381638e-2_f64) * t30663;
    let t30665 = F::new(1.0) / t30174;
    let t30668 = t151 * t593 * t30665 * t56;
    let t30669 = t30668 * t601;
    let t30670 = F::cast_from(0.36014175219178579057e-1_f64) * t30669;
    let t30671 = t30668 * t606;
    (t30658, t30659, t30661, t30664, t30670, t30671)
}
