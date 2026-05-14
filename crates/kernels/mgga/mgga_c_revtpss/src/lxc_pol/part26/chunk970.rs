//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 970/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk970<F: Float>(t25394: F, t26550: F, t2061: F, t25402: F, t7056: F, t10073: F, t26544: F, t7064: F, t7384: F, t887: F, t689: F, t7399: F, t786: F, t789: F, t231: F, t7398: F, t836: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t26551 = t26550 * t25394;
    let t26554 = t25402 * t2061;
    let t26555 = t7056 * t26554;
    let t26557 = 0.24093411633903331839e-3 * t10073 * t26555;
    let t26558 = t7064 * t26544;
    let t26560 = t7384 * t887;
    let t26561 = t689 * t26560;
    let t26563 = t786 * t7399;
    let t26564 = t26563 * t789;
    let t26567 = t7398 * t836 * t231;
    (t26551, t26554, t26555, t26557, t26558, t26560, t26561, t26563, t26564, t26567)
}
