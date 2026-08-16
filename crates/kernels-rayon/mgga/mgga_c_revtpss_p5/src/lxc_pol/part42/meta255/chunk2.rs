//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 976/1505 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk976(t114: f64, t1513: f64, t8311: f64, t109: f64, t55: f64, t655: f64, t1509: f64, t8315: f64, t69: f64, t8258: f64, t8267: f64, t8310: f64) -> (f64, f64, f64, f64) {
    let t115 = 1.0_f64 < t114;
    let t8395 = t8311 * t1513;
    let t8399 = t655 * t55 * t109;
    let t8402 = t8315 * t1509;
    let t8406 = piecewise3(t115, 0.0_f64, t8310 + t8258 * t8395 / 4.0_f64 + 5.0_f64 / 24.0_f64 * t69 * t8399 - 5.0_f64 / 24.0_f64 * t8267 * t8402);
    (t8395, t8399, t8402, t8406)
}
