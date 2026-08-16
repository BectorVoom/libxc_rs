//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 783/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk783(t219: f64, t5392: f64, t5395: f64, t3319: f64, t5371: f64, t1228: f64, t5366: f64, t1634: f64, t1636: f64, t516: f64, t518: f64) -> (f64, f64, f64, f64) {
    let t5397 = (t5392 + t5395) * t219;
    let t5401 = t3319 * t5371;
    let t5404 = t1228 * t5366;
    let t5407 = 6.0_f64 * t1634 * t1636 - 12.0_f64 * t516 * t5401 + 3.0_f64 * t516 * t5404 - t518 * t5397;
    (t5397, t5401, t5404, t5407)
}
