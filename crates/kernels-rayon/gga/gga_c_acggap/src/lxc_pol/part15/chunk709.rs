//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 709/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk709(t425: f64, t7614: f64, t431: f64, t438: f64, t7605: f64, t1966: f64, t377: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7615 = t7614 * t425;
    let t7616 = 0.16006300097412701803e-1_f64 * t7615;
    let t7617 = t7614 * t431;
    let t7622 = t7614 * t438;
    let t7624 = t7605 * t425;
    let t7625 = 0.34299214494455789578e-2_f64 * t7624;
    let t7628 = t7605 * t431;
    let t7629 = 0.17149607247227894789e-2_f64 * t7628;
    let t7630 = t377 * t1966;
    (t7616, t7617, t7622, t7625, t7629, t7630)
}
