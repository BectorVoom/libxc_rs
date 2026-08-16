//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2646/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2646(t46694: f64, t5686: f64, t14030: f64, t9744: f64, t13769: f64, t808: f64, t9736: f64, t13952: f64, t2689: f64, t13784: f64, t543: f64, t46825: f64, t9793: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t48685 = t46694 * t5686;
    let t48686 = 35.0_f64 / 24.0_f64 * t48685;
    let t48687 = t9744 * t14030;
    let t48690 = t9736 * t808 * t13769;
    let t48691 = 0.15246000842785598468e-3_f64 * t48690;
    let t48692 = t2689 * t13952;
    let t48694 = t13784 * t543;
    let t48696 = t9793 * t46825 * t48694;
    (t48686, t48687, t48691, t48692, t48694, t48696)
}
