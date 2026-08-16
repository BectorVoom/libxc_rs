//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 2033/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2033(t30: f64, t265: f64, t393: f64, t103658: f64, t103706: f64, t102867: f64, t102905: f64, t103574: f64, t103612: f64, t13312: f64, t1469: f64, t2078: f64, t2258: f64, t26626: f64, t28523: f64, t4186: f64, t45: f64, t606: f64, t7449: f64, t8040: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t103707 = t103658 + t103706;
    let t103708 = piecewise3(t394, 0.0_f64, t103707);
    let t103720 = piecewise3(t120, t102867 + t102905 + t103574 + t103612, t103708 * t45 / 2.0_f64 + t28523 * t606 + t8040 * t2258 / 2.0_f64 + t26626 * t1469 / 2.0_f64 + t7449 * t4186 + t2078 * t13312 / 2.0_f64);
    (t103707, t103720)
}
