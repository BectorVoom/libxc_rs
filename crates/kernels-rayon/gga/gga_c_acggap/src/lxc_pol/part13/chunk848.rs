//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 848/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk848(t1345: f64, t322: f64, t1662: f64, t301: f64, t467: f64, t495: f64, t811: f64, t694: f64, t7298: f64, t104: f64, t8020: f64, t2162: f64, t469: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t23745 = t1345 * t322;
    let t24589 = t301 * t1662;
    let t24605 = t1662 * t467;
    let t24623 = t495 * t811;
    let t29938 = t694 * t7298;
    let t29943 = t104 * t8020;
    let t29948 = t2162 * t469;
    (t23745, t24589, t24605, t24623, t29938, t29943, t29948)
}
