//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1539/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1539(t24042: f64, t994: f64, t23959: f64, t378: f64, t4746: f64, t6343: f64, t79862: f64, t1647: f64, t1678: f64, t6235: f64, t342: f64, t25026: f64, t3801: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t80810 = t994 * t24042;
    let t80833 = t23959 * t378;
    let t80901 = t4746 * t6343;
    let t80921 = t79862 * t378;
    let t80983 = t1647 * t6343;
    let t80992 = t6235 * t1678;
    let t81052 = t342 * t24042;
    let t81139 = t25026 * t3801;
    (t80810, t80833, t80901, t80921, t80983, t80992, t81052, t81139)
}
