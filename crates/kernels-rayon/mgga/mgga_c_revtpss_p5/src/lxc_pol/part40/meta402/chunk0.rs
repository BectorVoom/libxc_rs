//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1478/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1478(t114: f64, t31142: f64, t8315: f64, t2366: f64, t8311: f64, t104: f64, t2357: f64, t2358: f64, t2362: f64, t31035: f64, t31134: f64, t31135: f64, t31137: f64, t31139: f64, t8258: f64, t8267: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t115 = 1.0_f64 < t114;
    let t31143 = t8315 * t31142;
    let t31146 = t8311 * t2366;
    let t31149 = t104 * t2357;
    let t31150 = t31149 * t2358;
    let t31153 = t8315 * t2362;
    let t31157 = piecewise3(t115, 0.0_f64, -t31134 - 4.0_f64 / 3.0_f64 * t31135 + 10.0_f64 / 9.0_f64 * t31137 - 3.0_f64 / 4.0_f64 * t31035 * t31139 + 5.0_f64 / 6.0_f64 * t8258 * t31143 + t8258 * t31146 / 4.0_f64 - 5.0_f64 / 36.0_f64 * t8267 * t31150 - 5.0_f64 / 24.0_f64 * t8267 * t31153);
    (t31143, t31146, t31149, t31150, t31153, t31157)
}
