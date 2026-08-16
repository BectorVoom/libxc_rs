//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1147/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1147(t30: f64, t1468: f64, t9335: f64, t2: f64, t3833: f64, t580: f64, t605: f64, t22: f64, t2257: f64, t3834: f64, t513: f64, t5549: f64, t5552: f64, zeta_threshold: f64) -> (f64, f64) {
    let t31 = t30 <= zeta_threshold;
    let t13550 = t9335 * t1468;
    let t13553 = t3833 * t2;
    let t13554 = t580 * t605;
    let t13564 = piecewise3(t31, 0.0_f64, -8.0_f64 / 27.0_f64 * t13550 * t3834 + 16.0_f64 / 9.0_f64 * t13553 * t13554 + 4.0_f64 / 9.0_f64 * t5549 * t2257 + 8.0_f64 / 3.0_f64 * t513 * t580 - 8.0_f64 * t5552 * t22);
    (t13554, t13564)
}
