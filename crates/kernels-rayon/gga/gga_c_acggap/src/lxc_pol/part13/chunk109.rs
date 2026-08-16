//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 109/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk109(t147: f64, t330: f64, t154: f64, t19: f64, t56: f64, t124: f64) -> (f64, f64, f64) {
    let t332 = 7.0_f64 / 288.0_f64 * t330 * t147;
    let t334 = t56 * t154 * t19;
    let t335 = t124 * t334;
    (t332, t334, t335)
}
