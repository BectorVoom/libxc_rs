//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 977/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk977(t30546: f64, t8657: f64, t4198: f64, t7646: f64, t30601: f64, t30605: f64, t1061: f64, t535: f64, t7380: f64, t1165: f64, t33509: f64, t604: f64, t7346: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t34478 = t30546 * t8657;
    let t34481 = t4198 * t7646;
    let t34484 = t30601 / 64.0_f64;
    let t34485 = t30605 / 192.0_f64;
    let t34487 = t535 * t1061;
    let t34488 = t7380 * t34487;
    let t34492 = t7346 * t1165 * t604 * t33509;
    (t34478, t34481, t34484, t34485, t34487, t34488, t34492)
}
