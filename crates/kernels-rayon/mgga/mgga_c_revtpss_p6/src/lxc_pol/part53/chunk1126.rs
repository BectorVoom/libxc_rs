//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1126/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1126(t1470: f64, t644: f64, t6972: f64, t8442: f64, t640: f64, t36: f64, t606: f64, t7714: f64, t1493: f64, t33612: f64, t8621: f64, t37: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t125260 = t1470 * t644;
    let t125265 = t8442 * t1470 * t6972;
    let t125268 = t1470 * t640;
    let t125274 = t8442 * t7714 * t36 * t606;
    let t125279 = t1493 * t36 * t606;
    let t125294 = t8621 * t33612 * t6972;
    let t125312 = t37 * t606;
    (t125260, t125265, t125268, t125274, t125279, t125294, t125312)
}
