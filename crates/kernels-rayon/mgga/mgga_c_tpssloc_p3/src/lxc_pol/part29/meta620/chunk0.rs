//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2062/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2062(t24574: f64, t24860: f64, t24594: f64, t24847: f64, t974: f64, t27551: f64, t7327: f64, t135: f64, t7284: f64, t24853: f64, t24778: f64, t24762: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t86073 = t24574 * t24860;
    let t86076 = t24847 * t974 * t24594;
    let t86077 = t7327 * t27551;
    let t86094 = t24847 * t135 * t7284;
    let t86095 = t86094 * t24853;
    let t86106 = t24574 * t24778;
    let t86113 = t24574 * t24762;
    (t86073, t86076, t86077, t86094, t86095, t86106, t86113)
}
