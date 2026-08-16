//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1758/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1758(t2259: f64, t603: f64, t48: f64, t613: f64, t2275: f64, t43: f64, t239: f64, t2251: f64, t2258: f64, t2269: f64, t49: f64, t606: f64, t6968: f64) -> (f64, f64, f64, f64, f64) {
    let t25120 = t603 * t2259;
    let t25129 = t613 * t48;
    let t25132 = t43 * t2275;
    let t25137 = 88.0_f64 / 9.0_f64 * t239;
    let t25138 = 88.0_f64 / 9.0_f64 * t2269 * t49 - 40.0_f64 / 9.0_f64 * t25129 * t606 + 5.0_f64 / 18.0_f64 * t25132 * t2251 + 5.0_f64 / 6.0_f64 * t6968 * t2258 - t25137;
    (t25120, t25129, t25132, t25137, t25138)
}
