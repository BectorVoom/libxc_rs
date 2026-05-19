//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1058/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1058<F: Float>(t2471: F, t7388: F, t2061: F, t822: F, t25402: F, t7056: F, t10073: F, t2062: F, t2453: F, t2458: F, t11064: F, t2070: F) -> (F, F, F, F, F, F, F, F) {
    let t26538 = F::cast_from(0.13009920719177044025e-1_f64) * t7388 * t2471;
    let t26550 = t822 * t2061;
    let t26554 = t25402 * t2061;
    let t26555 = t7056 * t26554;
    let t26557 = F::cast_from(0.24093411633903331839e-3_f64) * t10073 * t26555;
    let t26576 = t2453 * t2062;
    let t26578 = F::cast_from(0.11565819519348392139e-2_f64) * t26576 * t2458;
    let t26590 = t2070 * t11064;
    (t26538, t26550, t26554, t26555, t26557, t26576, t26578, t26590)
}
