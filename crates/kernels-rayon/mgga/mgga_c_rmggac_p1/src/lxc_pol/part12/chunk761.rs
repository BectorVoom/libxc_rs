//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 761/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk761(t1341: f64, t357: f64, t638: f64, t7310: f64, t7254: f64, t7364: f64, t7243: f64, t1973: f64, t1965: f64, t7942: f64, t1969: f64, t1987: f64, t34881: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t35633 = t638 * t7310 * t357 * t1341;
    let t35637 = t7254 * t7364;
    let t35654 = t7254 * t7243;
    let t35655 = t35654 * t1973;
    let t35657 = t7942 * t1965;
    let t35658 = t35657 * t1969;
    let t35665 = t34881 * t1987;
    (t35633, t35637, t35654, t35655, t35658, t35665)
}
