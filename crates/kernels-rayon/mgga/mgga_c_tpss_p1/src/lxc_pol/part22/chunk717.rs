//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 717/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk717(t114: f64, t3532: f64, t630: f64, t2069: f64, t2070: f64, t3506: f64, t3509: f64, t69: f64) -> (f64, f64) {
    let t115 = 1.0_f64 < t114;
    let t3533 = t630 * t3532;
    let t3537 = piecewise3(t115, 0.0_f64, t2069 + t2070 / 3.0_f64 + t3506 / 3.0_f64 + t69 * t3509 / 4.0_f64 - t69 * t3533 / 8.0_f64);
    (t3533, t3537)
}
