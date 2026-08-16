//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 802/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk802(t552: f64, t8591: f64, t8576: f64, t8579: f64, t8580: f64, t8583: f64, t8586: f64) -> f64 {
    let t8592 = t8591 * t552;
    let t8594 = 0.28234466758480466999e-3_f64 * t8576 - 0.8673628188205199462e0_f64 * t8579 * t8580 + 0.57119737665102352616e0_f64 * t8583 * t8586 - 0.1859366460452550541e-3_f64 * t8592;
    t8594
}
