//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1205/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1205(t109: f64, t1845: f64, t6347: f64, t1851: f64, t5456: f64, t106944: f64, t106946: f64, t106948: f64, t84036: f64, t86586: f64, t96713: f64, t96721: f64, t107007: f64, t107015: f64, t107031: f64, t1807: f64, t20612: f64, t26224: f64, t26989: f64, t29286: f64, t568: f64, t84400: f64, t90551: f64, t90582: f64, t90642: f64, t97503: f64, t97509: f64) -> (f64, f64, f64, f64) {
    let t110 = 1.0_f64 < t109;
    let t107504 = t6347 * t1845;
    let t107571 = t1851 * t5456;
    let t107634 = piecewise3(t110, 0.0_f64, -t84036 - 22.0_f64 / 3.0_f64 * t86586 - 4.0_f64 * t96713 + 2.0_f64 * t96721 - 3.0_f64 / 2.0_f64 * t106944 + 3.0_f64 / 2.0_f64 * t106946 - t106948 / 4.0_f64);
    let t107694 = -0.31253747270116302294e0_f64 * t90551 - 18.0_f64 * t26224 * t26989 * t20612 + 0.9869604401089358619e-1_f64 * t107007 + 0.15626873635058151147e0_f64 * t90582 + 0.9869604401089358619e-1_f64 * t107015 - 0.9869604401089358619e-1_f64 * t97503 - t84400 + 3.0_f64 * t1807 * t29286 * t568 + 0.49348022005446793095e-1_f64 * t97509 - 0.19739208802178717238e0_f64 * t107031 + 0.49348022005446793095e-1_f64 * t90642;
    (t107504, t107571, t107634, t107694)
}
