//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1324/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1324(t22742: f64, t77: f64, t84: f64, t5825: f64, t22672: f64, t603: f64, t4173: f64, t5819: f64, t22738: f64, t76: f64, t38: f64, t85037: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t114305 = t77 * t84 * t22742;
    let t114311 = t77 * t84 * t5825;
    let t114313 = t603 * t22672;
    let t114322 = t4173 * t5819;
    let t114343 = t76 * t22738;
    let t114349 = t85037 * t38;
    (t114305, t114311, t114313, t114322, t114343, t114349)
}
