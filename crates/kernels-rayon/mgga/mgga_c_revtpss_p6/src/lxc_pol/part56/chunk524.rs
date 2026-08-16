//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 524/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk524(t1534: f64, t177: f64, t762: f64, t162: f64, t2611: f64, t1469: f64, t189: f64, t606: f64, t2623: f64, t2621: f64, t2628: f64, t2632: f64, t4307: f64, t4310: f64, t4313: f64, t4316: f64, t4394: f64, t4396: f64, t4397: f64) -> (f64, f64, f64, f64) {
    let t4398 = t1534 * t177;
    let t4399 = t4398 * t762;
    let t4400 = 0.5848223622634646207e0_f64 * t4399;
    let t4401 = t2611 * t162;
    let t4402 = t189 * t1469;
    let t4403 = t4402 * t606;
    let t4405 = 12.0_f64 * t4401 * t4403;
    let t4406 = 0.18311447306006545054e-3_f64 * t2623;
    let t4407 = t4307 + t4310 + t4313 + t4316 + t2632 + t2628 + t4394 + t4396 + t4397 - t4400 + t4405 + t2621 - t4406;
    (t4400, t4405, t4406, t4407)
}
