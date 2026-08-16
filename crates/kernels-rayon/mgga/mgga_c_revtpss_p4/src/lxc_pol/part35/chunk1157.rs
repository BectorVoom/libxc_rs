//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1157/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1157(t105945: f64, t7063: f64, t30105: f64, t689: f64, t1032: f64, t6888: f64, t1426: f64, t1955: f64, t786: f64, t6871: f64, t94429: f64, t22102: f64, t94423: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t106387 = t7063 * t105945;
    let t108138 = t30105 * t689;
    let t108277 = t6888 * t1032;
    let t108278 = t108277 * t1426;
    let t108279 = t7063 * t108278;
    let t108282 = t1955 * t108277;
    let t108379 = t786 * t108278;
    let t108516 = t94429 * t6871;
    let t108524 = t94423 * t22102;
    (t106387, t108138, t108279, t108282, t108379, t108516, t108524)
}
