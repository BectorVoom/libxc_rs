//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 895/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk895(t1184: f64, t30644: f64, t7433: f64, t7580: f64, t7728: f64, t1165: f64, t3529: f64, t7351: f64, t7426: f64, t7538: f64, t7720: f64, t7724: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t30645 = t30644 * t1184;
    let t30647 = t7433 * t7580;
    let t30649 = t7433 * t7728;
    let t30653 = t7426 * t1165 * t7351 * t3529;
    let t30655 = t7538 * t7720;
    let t30657 = t7538 * t7724;
    (t30645, t30647, t30649, t30653, t30655, t30657)
}
