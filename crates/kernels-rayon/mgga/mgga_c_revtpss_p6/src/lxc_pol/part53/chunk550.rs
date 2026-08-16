//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 550/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk550(t225: f64, t2718: f64, t213: f64, t2783: f64, t1568: f64, t233: f64, t869: f64, t689: f64, t72: f64, t686: f64, t874: f64, t822: f64) -> (f64, f64, f64, f64, f64) {
    let t4503 = t225 * t2718;
    let t4504 = t213 * t4503;
    let t4514 = t213 * t2783;
    let t4518 = t233 * t1568;
    let t4519 = t869 * t4518;
    let t4520 = t689 * t4519;
    let t4522 = t1568 * t72;
    let t4524 = t874 * t4522 * t686;
    let t4526 = t822 * t1568;
    (t4504, t4514, t4520, t4524, t4526)
}
