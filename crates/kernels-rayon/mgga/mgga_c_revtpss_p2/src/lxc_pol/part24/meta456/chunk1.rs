//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1425/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1425(t1606: f64, t9303: f64, t11384: f64, t1596: f64, t11465: f64, t1626: f64, t11298: f64, t11506: f64, t11408: f64, t1614: f64, t11449: f64, t11199: f64, t1646: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t52128 = t9303 * t1606;
    let t52224 = t1596 * t11384;
    let t52443 = t1626 * t11465;
    let t52508 = t1596 * t11298;
    let t52642 = t1626 * t11506;
    let t52812 = t1614 * t11408;
    let t52825 = t1614 * t11449;
    let t53014 = t1646 * t11199;
    (t52128, t52224, t52443, t52508, t52642, t52812, t52825, t53014)
}
