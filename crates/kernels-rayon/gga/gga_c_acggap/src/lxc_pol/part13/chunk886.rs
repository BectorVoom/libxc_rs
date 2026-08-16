//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 886/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk886(t1165: f64, t3759: f64, t7351: f64, t7426: f64, t3360: f64, t7646: f64, t3393: f64, t7361: f64, t7433: f64, t7353: f64, t1181: f64, t16548: f64, t599: f64, t7346: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t30463 = t7426 * t1165 * t7351 * t3759;
    let t30468 = t3360 * t7646;
    let t30469 = t30468 * t3393;
    let t30497 = t7433 * t7361;
    let t30507 = t7433 * t7353;
    let t30511 = t7346 * t1181 * t599 * t16548;
    (t30463, t30468, t30469, t30497, t30507, t30511)
}
