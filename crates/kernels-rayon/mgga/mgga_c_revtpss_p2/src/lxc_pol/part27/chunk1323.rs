//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1323/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1323(t45551: f64, t473: f64, t1243: f64, t2149: f64, t37885: f64, t1294: f64, t21471: f64, t3555: f64, t7627: f64, t1209: f64, t26884: f64, t26921: f64, t7648: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t97377 = t45551 * t473;
    let t97397 = t2149 * t37885 * t1243;
    let t97398 = t21471 * t1294;
    let t97402 = t3555 * t7627;
    let t97419 = t1209 * t26884;
    let t97422 = t7648 * t26921;
    (t97377, t97397, t97398, t97402, t97419, t97422)
}
