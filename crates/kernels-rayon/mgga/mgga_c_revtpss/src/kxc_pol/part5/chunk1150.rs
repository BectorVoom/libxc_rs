//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1150/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1150(t45: f64, t57: f64, t5819: f64, t633: f64, t5825: f64, t80: f64, t18281: f64, t4186: f64, t4328: f64, t606: f64, t766: f64, t637: f64, t83: f64, t4335: f64, t770: f64, zeta_threshold: f64) -> (f64, f64) {
    let t151 = t45 <= zeta_threshold;
    let t155 = t57 <= zeta_threshold;
    let t18367 = t633 * t5819;
    let t18372 = t80 * t5825;
    let t18378 = piecewise3(t151, 0.0_f64, 8.0_f64 / 27.0_f64 * t18367 * t606 - 4.0_f64 / 9.0_f64 * t4328 * t4186 - 2.0_f64 / 9.0_f64 * t18372 * t606 + 2.0_f64 / 3.0_f64 * t766 * t18281);
    let t18379 = t637 * t5819;
    let t18384 = t83 * t5825;
    let t18390 = piecewise3(t155, 0.0_f64, -8.0_f64 / 27.0_f64 * t18379 * t606 - 4.0_f64 / 9.0_f64 * t4335 * t4186 - 2.0_f64 / 9.0_f64 * t18384 * t606 - 2.0_f64 / 3.0_f64 * t770 * t18281);
    (t18378, t18390)
}
