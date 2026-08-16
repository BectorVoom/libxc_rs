//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 82/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk82(t229: f64, t88: f64, t37: f64, t34: f64, t41: f64) -> (f64, f64, f64, f64) {
    let t230 = t229 * t88;
    let t231 = 4.0_f64 * t230;
    let t232 = 1.0_f64 / t37;
    let t233 = t34 * t232;
    let t234 = t41 - t233;
    (t231, t232, t233, t234)
}
