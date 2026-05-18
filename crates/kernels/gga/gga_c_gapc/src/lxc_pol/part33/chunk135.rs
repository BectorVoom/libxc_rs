//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 135/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk135<F: Float>(t104: F, t442: F, t14: F, t23: F, t402: F, t70: F, t105: F, t107: F, t108: F, t405: F, t438: F, t73: F) -> (F, F, F, F, F) {
    let t443 = t104 * t104;
    let t444 = t443 * t443;
    let t445 = t444 * t104;
    let t446 = t442 * t445;
    let t447 = t23 * t14;
    let t451 = t70 * t402;
    let t457 = F::new(0.13140859333333333333e-2) * t105 * t438 * t108 - F::new(0.98556444999999999995e-3) * t446 * t447 * t108 - F::new(0.19711288999999999999e-2) * t105 * t107 * t451 - F::new(4.0) * t73 * t405;
    (t445, t446, t447, t451, t457)
}
