//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 212/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk212(t760: f64, t762: f64, t206: f64, t262: f64, t78: f64, t81: f64, t212: f64, t251: f64, t225: f64, t257: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t764 = 0.5848223622634646207e0_f64 * t760 * t762;
    let t765 = t206 * t262;
    let t766 = 1.0_f64 / t78;
    let t770 = 1.0_f64 / t81;
    let t779 = t212 * t251;
    let t780 = t225 * t257;
    (t764, t765, t766, t770, t779, t780)
}
