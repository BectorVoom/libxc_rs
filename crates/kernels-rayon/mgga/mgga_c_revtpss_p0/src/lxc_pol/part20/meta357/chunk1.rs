//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1301/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1301(t39597: f64, t786: f64, t10665: f64, t675: f64, t10871: f64, t268: f64, t10530: f64, t2723: f64, t4503: f64, t860: f64, t10532: f64, t10542: f64, t10547: f64) -> (f64, f64, f64, f64, f64) {
    let t39598 = t786 * t39597;
    let t39599 = t675 * t10665;
    let t39602 = t39598 * t268 * t39599 * t10871;
    let t39606 = t10530 * t268 * t39599 * t2723;
    let t39608 = t4503 * t860;
    let t39609 = t786 * t39608;
    let t39610 = t39609 * t10532;
    let t39612 = t10542 * t10547;
    (t39599, t39602, t39606, t39610, t39612)
}
