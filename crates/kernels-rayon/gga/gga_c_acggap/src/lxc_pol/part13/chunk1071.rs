//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1071/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1071(t10146: f64, t167: f64, t576: f64, t137: f64, t3300: f64, t4263: f64, t30407: f64, t31097: f64, t495: f64, t7325: f64, t4410: f64, t7561: f64) -> (f64, f64, f64) {
    let t34691 = t576 * t167 * t10146;
    let t34692 = t3300 * t137;
    let t34694 = t34691 * t34692 * t4263;
    let t34698 = t30407 * t31097 * t7325 * t495;
    let t34700 = t7561 * t4410;
    (t34694, t34698, t34700)
}
