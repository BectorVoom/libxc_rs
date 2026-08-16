//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1976/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1976(t1518: f64, t8233: f64, t1519: f64, t2165: f64, t29427: f64, t29590: f64, t29993: f64, t29998: f64, t30007: f64, t30015: f64, t30113: f64, t30125: f64, t30127: f64, t30130: f64, t30154: f64, t30156: f64, t30158: f64, t30951: f64, t30959: f64, t4248: f64, t569: f64, t5887: f64, t5921: f64, t651: f64, t6934: f64, t7586: f64, t8158: f64) -> (f64, f64) {
    let t30963 = t8233 * t1518;
    let t30973 = -4.0_f64 * t1519 * t29427 + t2165 * t6934 - 2.0_f64 * t30951 * t651 + t30959 * t569 - 4.0_f64 * t30963 * t651 - 4.0_f64 * t4248 * t8158 - 4.0_f64 * t5887 * t7586 - 2.0_f64 * t5921 * t7586 - t29590 - t29993 - t29998 - t30007 + t30015 + t30113 - t30125 - t30127 - t30130 - t30154 - t30156 - t30158;
    (t30963, t30973)
}
