//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 895/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk895(t25411: f64, t27186: f64, t213: f64, t7759: f64, t25431: f64, t212: f64, t780: f64, t689: f64, t1032: f64, t1568: f64, t1955: f64, t7760: f64, t786: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t27187 = t25411 * t27186;
    let t27189 = t213 * t7759;
    let t27192 = t25431 * t27186;
    let t27194 = t212 * t7759;
    let t27195 = t27194 * t780;
    let t27196 = t689 * t27195;
    let t27198 = t1568 * t1032;
    let t27199 = t1955 * t27198;
    let t27202 = t786 * t7760;
    (t27187, t27189, t27192, t27196, t27198, t27199, t27202)
}
