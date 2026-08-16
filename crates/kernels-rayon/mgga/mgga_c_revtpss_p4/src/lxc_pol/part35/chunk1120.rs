//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1120/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1120(t25877: f64, t94390: f64, t46361: f64, t545: f64, t1032: f64, t9656: f64, t25875: f64, t25894: f64, t1426: f64, t9990: f64, t7282: f64, t9646: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t94589 = t94390 * t25877;
    let t94656 = t46361 * t545;
    let t94667 = t1032 * t9656;
    let t94668 = t94667 * t545;
    let t94669 = t25875 * t94668;
    let t94674 = t25894 * t94668;
    let t94683 = t1426 * t9990;
    let t94696 = t9646 * t7282;
    (t94589, t94656, t94669, t94674, t94683, t94696)
}
