//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 813/1327 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk813<F: Float>(t329: F, t9721: F, t3407: F, t3403: F, t197: F, t7807: F, t3336: F, t333: F, t474: F, t2482: F, t667: F, t6851: F) -> (F, F, F, F, F, F) {
    let t9722 = t9721 * t329;
    let t9723 = t9722 * t3407;
    let t9724 = t3403 * t9723;
    let t9730 = t197 * t7807;
    let t9731 = t3336 * t9730;
    let t9733 = t474 * t333;
    let t9734 = t2482 * t9733;
    let t9736 = t6851 * t667;
    (t9722, t9723, t9724, t9731, t9734, t9736)
}
