//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1524/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1524(t23492: f64, t698: f64, t23471: f64, t23495: f64, t23510: f64, t23507: f64, t23475: f64, t23663: f64, t914: f64, t23798: f64, t945: f64, t23811: f64, t964: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t77663 = t698 * t23492;
    let t77667 = t698 * t23471;
    let t77736 = t698 * t23495;
    let t77804 = t698 * t23510;
    let t77806 = t698 * t23507;
    let t77858 = t698 * t23475;
    let t78097 = t23663 * t914;
    let t78108 = t23798 * t945;
    let t78111 = t23811 * t964;
    (t77663, t77667, t77736, t77804, t77806, t77858, t78097, t78108, t78111)
}
