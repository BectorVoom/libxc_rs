//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 636/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk636(t30: f64, t33: f64, t525: f64, t605: f64, t2257: f64, t513: f64, t527: f64, t1113: f64, t3351: f64, t516: f64, t162: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64) {
    let t31 = t30 <= zeta_threshold;
    let t34 = t33 <= zeta_threshold;
    let t3833 = 1.0_f64 / t525;
    let t3834 = t605 * t605;
    let t3840 = piecewise3(t31, 0.0_f64, 4.0_f64 / 9.0_f64 * t3833 * t3834 + 4.0_f64 / 3.0_f64 * t513 * t2257);
    let t3841 = 1.0_f64 / t527;
    let t3842 = t1113 * t1113;
    let t3848 = piecewise3(t34, 0.0_f64, 4.0_f64 / 9.0_f64 * t3841 * t3842 + 4.0_f64 / 3.0_f64 * t516 * t3351);
    let t3850 = (t3840 + t3848) * t162;
    (t3833, t3834, t3841, t3842, t3850)
}
