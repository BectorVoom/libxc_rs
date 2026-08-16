//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1447/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1447(t141: f64, t2908: f64, t41325: f64, t41310: f64, t930: f64, t41318: f64, t9303: f64, t931: f64, t41308: f64, t41312: f64, t41320: f64, t41327: f64, t41330: f64, t41332: f64, t41334: f64, t41336: f64, t41365: f64, t41367: f64) -> (f64, f64, f64, f64, f64) {
    let t41433 = t141 * t2908 * t41325;
    let t41436 = t141 * t930 * t41310;
    let t41439 = t141 * t930 * t41318;
    let t41441 = t9303 * t931;
    let t41443 = -0.24154e1_f64 * t41365 + 0.80513333333333333333e0_f64 * t41367 + 0.24154e1_f64 * t41308 + 0.72462e1_f64 * t41312 + 0.181155e1_f64 * t41320 - 0.60384999999999999999e0_f64 * t41327 - 0.80513333333333333336e0_f64 * t41330 - 0.53675555555555555556e0_f64 * t41332 + 0.40256666666666666668e0_f64 * t41334 + 0.44729629629629629629e0_f64 * t41336 - 0.82785e-1_f64 * t41433 + 0.198684e1_f64 * t41436 + 0.49671e0_f64 * t41439 + 0.98115555555555555556e0_f64 * t41441;
    (t41433, t41436, t41439, t41441, t41443)
}
