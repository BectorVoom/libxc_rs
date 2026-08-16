//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 980/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk980(t30937: f64, t8450: f64, t30638: f64, t10098: f64, t8462: f64, t8653: f64, t30655: f64, t30407: f64, t30408: f64, t30409: f64, t495: f64, t30402: f64, t506: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t34561 = t30937 * t8450;
    let t34566 = 35.0_f64 / 216.0_f64 * t30638;
    let t34569 = t10098 * t8462;
    let t34570 = t34569 * t8653;
    let t34575 = 0.42874018118069736972e-3_f64 * t30655;
    let t34578 = t30407 * t30408 * t30409 * t495;
    let t34582 = t30407 * t30402 * t30409 * t506;
    (t34561, t34566, t34569, t34570, t34575, t34578, t34582)
}
