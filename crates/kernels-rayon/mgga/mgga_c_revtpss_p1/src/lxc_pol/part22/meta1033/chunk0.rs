//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3616/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3616(t20343: f64, t698: f64, t20346: f64, t141: f64, t3417: f64, t68355: f64, t12254: f64, t68340: f64, t1134: f64, t5079: f64, t16851: f64, t16854: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t68548 = t698 * t20343;
    let t68550 = t698 * t20346;
    let t68553 = t141 * t3417 * t68355;
    let t68556 = t141 * t12254 * t68340;
    let t68558 = t1134 * t5079;
    let t68559 = t16851 * t68558;
    let t68561 = t16854 * t68558;
    (t68548, t68550, t68553, t68556, t68559, t68561)
}
