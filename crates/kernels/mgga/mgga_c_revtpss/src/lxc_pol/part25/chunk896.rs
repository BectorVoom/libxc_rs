//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 896/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk896<F: Float>(t11349: F, t11378: F, t935: F, t915: F, t2922: F, t913: F, t275: F, t290: F, t2925: F, t11300: F, t3022: F, t3030: F, t3034: F, t3006: F, t3011: F, t4733: F) -> (F, F, F, F, F) {
    let t11379 = t11349 + t11378;
    let t11380 = t11379 * t935;
    let t11382 = 1.0 * t915 * t11380;
    let t11384 = 1.0 / t2922 / t913;
    let t11385 = t275 * t11384;
    let t11387 = 1.0 / t2925 / t290;
    let t11388 = t11300 * t11387;
    let t11390 = 0.51726012919273400301e3 * t11385 * t11388;
    let t11392 = 0.17544670867903938621e1 * t3022 * t3030;
    let t11394 = 0.51947577317044391276e2 * t3022 * t3034;
    let t11396 = t3011 * t3006 * t4733;
    (t11382, t11390, t11392, t11394, t11396)
}
