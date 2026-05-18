//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 835/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk835<F: Float>(t197: F, t7807: F, t3336: F, t333: F, t474: F, t2482: F, t667: F, t6851: F, t4043: F, t311: F, t134: F, t959: F) -> (F, F, F, F) {
    let t9730 = t197 * t7807;
    let t9731 = t3336 * t9730;
    let t9733 = t474 * t333;
    let t9734 = t2482 * t9733;
    let t9736 = t6851 * t667;
    let t9737 = t9736 * t4043;
    let t9738 = t311 * t9737;
    let t9739 = t134 * t959;
    (t9731, t9734, t9738, t9739)
}
