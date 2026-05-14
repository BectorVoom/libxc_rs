//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 953/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk953<F: Float>(t26506: F, t7064: F, t136: F, t2066: F, t2457: F, t25299: F, t25305: F, t7058: F, t2471: F, t7388: F, t2061: F, t822: F, t25402: F, t7056: F, t10073: F, t2062: F, t2453: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t26508 = 0.17135234354032049604e-1 * t7064 * t26506;
    let t26518 = t2066 * t136;
    let t26519 = t26518 * t2457;
    let t26521 = 0.17135234354032049604e-2 * t25299 * t26519;
    let t26534 = 0.22849835011101738147e-2 * t25305 * t26519;
    let t26536 = 0.96373646535613327357e-2 * t7058 * t26506;
    let t26538 = 0.13009920719177044025e-1 * t7388 * t2471;
    let t26550 = t822 * t2061;
    let t26554 = t25402 * t2061;
    let t26555 = t7056 * t26554;
    let t26557 = 0.24093411633903331839e-3 * t10073 * t26555;
    let t26576 = t2453 * t2062;
    (t26508, t26518, t26519, t26521, t26534, t26536, t26538, t26550, t26554, t26555, t26557, t26576)
}
