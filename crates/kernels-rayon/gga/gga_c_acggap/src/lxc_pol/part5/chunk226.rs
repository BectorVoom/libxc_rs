//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 226/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk226(t761: f64, t779: f64, t777: f64, t132: f64, t265: f64, t264: f64, t80: f64) -> (f64, f64, f64, f64, f64) {
    let t780 = t761 * t779;
    let t781 = t777 * t780;
    let t782 = 0.16081979498692535067e2_f64 * t781;
    let t786 = t132 * t265;
    let t790 = t264 * t80;
    let t791 = 1.0_f64 / t790;
    (t780, t782, t786, t790, t791)
}
