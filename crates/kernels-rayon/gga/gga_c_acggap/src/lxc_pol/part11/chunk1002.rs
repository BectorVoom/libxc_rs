//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1002/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1002(t119: f64, t8993: f64, t1181: f64, t5258: f64, t604: f64, t7575: f64, t1165: f64, t4930: f64, t7351: f64, t1432: f64, t30147: f64, t30148: f64, t7842: f64) -> (f64, f64, f64, f64) {
    let t33818 = t119 * t8993;
    let t33823 = t7575 * t1181 * t604 * t5258;
    let t33827 = t7575 * t1165 * t7351 * t4930;
    let t33831 = t30147 * t7842 * t30148 * t1432;
    (t33818, t33823, t33827, t33831)
}
