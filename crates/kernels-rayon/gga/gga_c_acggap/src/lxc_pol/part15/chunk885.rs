//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 885/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk885(t301: f64, t30407: f64, t30408: f64, t30409: f64, t30402: f64, t360: f64, t172: f64, t2066: f64) -> (f64, f64, f64) {
    let t30412 = t30407 * t30408 * t30409 * t301;
    let t30416 = t30407 * t30402 * t30409 * t360;
    let t30418 = t2066 * t172;
    (t30412, t30416, t30418)
}
