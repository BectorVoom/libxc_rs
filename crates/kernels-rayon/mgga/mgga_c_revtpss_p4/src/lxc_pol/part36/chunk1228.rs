//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1228/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1228(t2453: f64, t4086: f64, t64: f64, t2018: f64, t40688: f64, t46808: f64, t7256: f64, t9784: f64, t25877: f64, t94390: f64, t7285: f64, t9288: f64) -> (f64, f64, f64, f64, f64) {
    let t94564 = t2453 * t4086 * t64;
    let t94568 = t40688 * t2018 * t46808;
    let t94569 = 0.22589491248727328397e-6_f64 * t94568;
    let t94570 = t9784 * t7256;
    let t94571 = 0.14450132032386466905e-2_f64 * t94570;
    let t94589 = t94390 * t25877;
    let t94600 = t7285 * t9288;
    (t94564, t94569, t94571, t94589, t94600)
}
