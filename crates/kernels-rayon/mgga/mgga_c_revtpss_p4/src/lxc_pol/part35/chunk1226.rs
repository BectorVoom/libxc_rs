//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1226/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1226(t115493: f64, t115521: f64, t115551: f64, t115592: f64, t115614: f64, t115637: f64, t115658: f64, t115744: f64, t892: f64, t102888: f64, t103586: f64, t113097: f64, t113100: f64, t113104: f64, t113108: f64, t113115: f64, t113123: f64, t113428: f64, t113433: f64, t113441: f64, t1940: f64, t2071: f64, t2072: f64, t2403: f64, t26425: f64, t28291: f64, t28472: f64, t29591: f64, t29599: f64, t29602: f64, t29606: f64, t29713: f64, t30: f64, t4541: f64, t8020: f64) -> (f64, f64, f64) {
    let t115747 = t115493 + t115521 + t115551 + t115592 + t115614 + t115637 + t115658 + t115744;
    let t115748 = t115747 * t892;
    let t115763 = -9.0_f64 * t102888 * t29599 + 3.0_f64 * t28472 * t113108 + 9.0_f64 * t2403 * t8020 * t29602 + 3.0_f64 * t1940 * t103586 * t29713 - 9.0_f64 / 2.0_f64 * t26425 * t113104 - 9.0_f64 * t26425 * t113115 - 9.0_f64 / 2.0_f64 * t26425 * t113433 + 3.0_f64 * t113123 * t2072 - 9.0_f64 * t28291 * t113097 + 9.0_f64 * t28291 * t113100 + t1940 * t115748 * t30 / 2.0_f64 + 9.0_f64 / 2.0_f64 * t2403 * t8020 * t29606 + 9.0_f64 * t26425 * t113441 + 3.0_f64 / 2.0_f64 * t2403 * t2071 * t113428 + 9.0_f64 * t4541 * t8020 * t29591;
    (t115747, t115748, t115763)
}
