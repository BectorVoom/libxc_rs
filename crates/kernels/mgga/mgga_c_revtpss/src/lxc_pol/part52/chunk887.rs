//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 887/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk887<F: Float>(t26543: F, t686: F, t7058: F, t213: F, t7398: F, t2061: F, t822: F, t25402: F, t7056: F, t10073: F, t7064: F, t7384: F, t887: F) -> (F, F, F, F, F, F) {
    let t26544 = t26543 * t686;
    let t26545 = t7058 * t26544;
    let t26547 = t213 * t7398;
    let t26550 = t822 * t2061;
    let t26554 = t25402 * t2061;
    let t26555 = t7056 * t26554;
    let t26557 = F::cast_from(0.24093411633903331839e-3_f64) * t10073 * t26555;
    let t26558 = t7064 * t26544;
    let t26560 = t7384 * t887;
    (t26545, t26547, t26550, t26557, t26558, t26560)
}
