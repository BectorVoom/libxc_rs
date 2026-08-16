//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1105/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1105(t30993: f64, t1165: f64, t20138: f64, t604: f64, t7413: f64, t1992: f64, t30127: f64, t7842: f64, t8791: f64, t1181: f64, t33509: f64, t599: f64, t7346: f64) -> (f64, f64, f64, f64) {
    let t35167 = 0.19055119163586549766e-2_f64 * t30993;
    let t35172 = t7413 * t1165 * t604 * t20138;
    let t35176 = t30127 * t7842 * t1992 * t8791;
    let t35180 = t7346 * t1181 * t599 * t33509;
    (t35167, t35172, t35176, t35180)
}
