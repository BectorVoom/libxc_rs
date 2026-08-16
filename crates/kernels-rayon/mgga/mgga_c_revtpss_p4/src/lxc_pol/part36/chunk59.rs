//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 59/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk59(t128: f64, t131: f64, t134: f64, t141: f64, t130: f64) -> (f64, f64, f64, f64) {
    let t143 = 0.379785e1_f64 * t131 + 0.8969e0_f64 * t128 + 0.204775e0_f64 * t134 + 0.123235e0_f64 * t141;
    let t146 = 1.0_f64 + 0.16081979498692535067e2_f64 / t143;
    let t147 = f64::ln(t146);
    let t149 = 0.621814e-1_f64 * t130 * t147;
    (t143, t146, t147, t149)
}
