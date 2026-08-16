//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 181/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk181(t5: f64, t506: f64, t129: f64, t145: f64, t369: f64, t371: f64, t504: f64) -> (f64, f64, f64) {
    let t507 = t5 * t506;
    let t509 = t129 * t507 * t145;
    let t513 = -t369 - 0.36675e0_f64 * t504 + t371;
    (t507, t509, t513)
}
