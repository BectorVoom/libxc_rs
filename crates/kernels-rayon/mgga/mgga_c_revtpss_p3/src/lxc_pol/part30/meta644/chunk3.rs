//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2264/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2264(t104135: f64, t104153: f64, t104433: f64, t105712: f64, t105724: f64, t105734: f64, t105741: f64, t105756: f64, t101568: f64, t101570: f64, t101572: f64, t101576: f64, t101578: f64, t101583: f64, t101586: f64, t101590: f64, t101594: f64, t101598: f64, t101601: f64, t101606: f64, t18204: f64, t18211: f64, t2170: f64, t4165: f64, t573: f64, t5805: f64, t7696: f64, t8245: f64, param_d: f64) -> (f64, f64) {
    let t105759 = t104135 + t104153 + t104433 + t105712 + t105724 + t105734 + t105741 + t105756;
    let t105762 = t105759 * t573 * param_d + 6.0_f64 * t18204 * t2170 + 6.0_f64 * t18211 * t2170 + 3.0_f64 * t4165 * t8245 + 6.0_f64 * t5805 * t7696 + t101568 + t101570 + t101572 + t101576 + t101578 + t101583 + t101586 + t101590 + t101594 + t101598 + t101601 + t101606;
    (t105759, t105762)
}
