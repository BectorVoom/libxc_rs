//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1480/1505 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1480(t5891: f64, t8311: f64, t1513: f64, t31429: f64, t1509: f64, t8315: f64, t5915: f64, t109: f64, t1479: f64, t655: f64, t31433: f64, t31149: f64, t5907: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t31626 = t8311 * t5891;
    let t31629 = t31429 * t1513;
    let t31632 = t1513 * t1509;
    let t31633 = t8315 * t31632;
    let t31636 = t8311 * t5915;
    let t31640 = t655 * t1479 * t109;
    let t31643 = t31433 * t1509;
    let t31646 = t31149 * t5907;
    (t31626, t31629, t31633, t31636, t31640, t31643, t31646)
}
