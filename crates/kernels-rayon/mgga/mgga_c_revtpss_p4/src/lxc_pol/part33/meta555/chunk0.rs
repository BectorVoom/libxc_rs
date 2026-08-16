//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1945/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1945(t30: f64, t6079: f64, t1468: f64, t1583: f64, t6075: f64, t1940: f64, t1963: f64, t2403: f64, t25206: f64, t25445: f64, t27368: f64, t29592: f64, t29599: f64, t29602: f64, t29606: f64, t29705: f64, t4541: f64, t5824: f64, t7091: f64, t7749: f64, t7783: f64, t7787: f64) -> (f64, f64, f64, f64) {
    let t29713 = t30 * t6079;
    let t29716 = t1468 * t1583;
    let t29719 = t30 * t6075;
    let t29726 = 3.0_f64 * t4541 * t29592 + 3.0_f64 * t2403 * t7783 * t7749 - 3.0_f64 * t25206 * t29599 + 3.0_f64 * t2403 * t1963 * t29602 + 3.0_f64 / 2.0_f64 * t2403 * t1963 * t29606 + t1940 * t29705 * t30 / 2.0_f64 - t1940 * t27368 * t7787 + t1940 * t7783 * t1468 + t1940 * t25445 * t29713 - t1940 * t7091 * t29716 - t1940 * t7091 * t29719 / 2.0_f64 + t1940 * t1963 * t5824 / 2.0_f64;
    (t29713, t29716, t29719, t29726)
}
