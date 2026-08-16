//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 225/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk225(t45: f64, t57: f64, t760: f64, t762: f64, t206: f64, t262: f64, t78: f64, t606: f64, t81: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64) {
    let t151 = t45 <= zeta_threshold;
    let t155 = t57 <= zeta_threshold;
    let t764 = 0.5848223622634646207e0_f64 * t760 * t762;
    let t765 = t206 * t262;
    let t766 = 1.0_f64 / t78;
    let t769 = piecewise3(t151, 0.0_f64, 2.0_f64 / 3.0_f64 * t766 * t606);
    let t770 = 1.0_f64 / t81;
    let t773 = piecewise3(t155, 0.0_f64, -2.0_f64 / 3.0_f64 * t770 * t606);
    let t775 = t769 / 2.0_f64 + t773 / 2.0_f64;
    (t764, t765, t766, t770, t775)
}
