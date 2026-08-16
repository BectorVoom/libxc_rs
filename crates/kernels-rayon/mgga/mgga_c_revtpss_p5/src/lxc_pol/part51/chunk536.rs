//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 536/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk536(t198: f64, t205: f64, t1544: f64, t262: f64, t1583: f64, t892: f64, t2404: f64, t2411: f64, t1940: f64, t207: f64, t2403: f64, t2621: f64, t2628: f64, t2632: f64, t4316: f64, t4343: f64, t4394: f64, t4396: f64, t4397: f64, t4400: f64, t4405: f64, t4406: f64, t4537: f64, t765: f64, t775: f64, t890: f64) -> (f64, f64) {
    let t4541 = t198 * t205;
    let t4542 = t262 * t1544;
    let t4546 = t1583 * t892;
    let t4553 = t2404 * t1544;
    let t4556 = t1583 * t2411;
    let t4559 = t198 * t207 * t4537 * t892 - t1940 * t4556 * t890 + 3.0_f64 * t198 * t4343 * t765 + 3.0_f64 * t2403 * t4546 * t775 + 6.0_f64 * t4541 * t4542 * t775 + 3.0_f64 * t2403 * t4553 + t2621 + t2628 + t2632 + t4316 + t4394 + t4396 + t4397 - t4400 + t4405 - t4406;
    (t4541, t4559)
}
