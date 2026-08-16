//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1477/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1477(t10208: f64, t69: f64, t2195: f64, t2289: f64, t31027: f64, t8312: f64, t31032: f64, t8316: f64, t2340: f64, t8311: f64, t661: f64, t665: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t31035 = t69 * t10208;
    let t31134 = 11.0_f64 / 9.0_f64 * t2289 * t2195;
    let t31135 = t31027 * t8312;
    let t31137 = t31032 * t8316;
    let t31139 = t8311 * t2340;
    let t31142 = t665 * t661;
    (t31035, t31134, t31135, t31137, t31139, t31142)
}
