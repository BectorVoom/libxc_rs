//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 674/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk674(t57: f64, t1469: f64, t83: f64, t4186: f64, t606: f64, t770: f64, t4334: f64, zeta_threshold: f64) -> (f64, f64) {
    let t155 = t57 <= zeta_threshold;
    let t4335 = t83 * t1469;
    let t4341 = piecewise3(t155, 0.0_f64, -2.0_f64 / 9.0_f64 * t4335 * t606 - 2.0_f64 / 3.0_f64 * t770 * t4186);
    let t4343 = t4334 / 2.0_f64 + t4341 / 2.0_f64;
    (t4335, t4343)
}
