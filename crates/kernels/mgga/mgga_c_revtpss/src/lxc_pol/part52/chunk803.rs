//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 803/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk803<F: Float>(t2061: F, t822: F, t25402: F, t7056: F, t10073: F, t26544: F, t7064: F, t7384: F, t887: F, t689: F, t7399: F, t786: F, t789: F, t2062: F, t2453: F, t2458: F) -> (F, F, F, F, F, F) {
    let t26550 = t822 * t2061;
    let t26554 = t25402 * t2061;
    let t26555 = t7056 * t26554;
    let t26557 = 0.24093411633903331839e-3 * t10073 * t26555;
    let t26558 = t7064 * t26544;
    let t26560 = t7384 * t887;
    let t26561 = t689 * t26560;
    let t26563 = t786 * t7399;
    let t26564 = t26563 * t789;
    let t26576 = t2453 * t2062;
    let t26578 = 0.11565819519348392139e-2 * t26576 * t2458;
    (t26550, t26557, t26558, t26561, t26564, t26578)
}
