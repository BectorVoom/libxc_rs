//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1474/1505 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1474(t4287: f64, t8311: f64, t625: f64, t8399: f64, t109: f64, t55: f64, t665: f64, t108: f64, t661: f64, t31032: f64, t8402: f64, t1509: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t31424 = t8311 * t4287;
    let t31427 = t625 * t8399;
    let t31429 = t55 * t109;
    let t31430 = t31429 * t665;
    let t31433 = t55 * t108;
    let t31434 = t31433 * t661;
    let t31437 = t31032 * t8402;
    let t31439 = t1509 * t665;
    (t31424, t31427, t31429, t31430, t31433, t31434, t31437, t31439)
}
