//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 1296/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk1296(t10245: f64, t2531: f64, t918: f64, t11512: f64, t2208: f64, t22672: f64, t2580: f64, t1056: f64, t3217: f64, t7371: f64, t2207: f64, t474: f64) -> (f64, f64, f64, f64) {
    let t35992 = t10245 * t918 * t2531;
    let t35996 = t22672 * t2208 * t11512 * t2580;
    let t35999 = t3217 * t1056 * t7371;
    let t36003 = t22672 * t2207 * t474 * t2580;
    (t35992, t35996, t35999, t36003)
}
