//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1178/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1178(t27126: f64, t8461: f64, t1583: f64, t7086: f64, t27383: f64, t1544: f64, t25207: f64, t605: f64, t7782: f64, t890: f64, t1468: f64, t775: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t125950 = 2.0_f64 * t27126 * t8461;
    let t125961 = t1583 * t7086;
    let t125962 = t27383 * t125961;
    let t125984 = t1544 * t7086;
    let t125985 = t25207 * t125984;
    let t126007 = t605 * t7782;
    let t126017 = t7782 * t890;
    let t126018 = t27383 * t126017;
    let t126027 = t1468 * t7086;
    let t126030 = t7782 * t775;
    (t125950, t125961, t125962, t125984, t125985, t126007, t126017, t126018, t126027, t126030)
}
