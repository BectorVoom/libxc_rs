//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1973/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1973(t28063: f64, t651: f64, t22496: f64, t8717: f64, t25082: f64, t1469: f64, t25129: f64, t25132: f64, t25137: f64, t4181: f64, t4186: f64, t6968: f64) -> (f64, f64, f64, f64) {
    let t28065 = 2.0_f64 * t651 * t28063;
    let t28067 = t8717 * t22496;
    let t28069 = 3.0_f64 * t25082 * t28067;
    let t28076 = -20.0_f64 / 9.0_f64 * t25129 * t1469 + 5.0_f64 / 18.0_f64 * t25132 * t4181 + 5.0_f64 / 6.0_f64 * t6968 * t4186 - t25137;
    (t28065, t28067, t28069, t28076)
}
