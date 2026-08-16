//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1229/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1229(t93303: f64, t93314: f64, t25386: f64, t93280: f64, t93282: f64, t786: f64, t860: f64, t25410: f64, t25413: f64, t7064: f64, t93150: f64, t25375: f64, t93311: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t93315 = t93314 * t93303;
    let t93317 = t25386 * t93280;
    let t93318 = t93317 * t93282;
    let t93320 = t786 * t860;
    let t93321 = t93320 * t25410;
    let t93322 = t93321 * t25413;
    let t93324 = t7064 * t93150;
    let t93326 = t25375 * t93311;
    (t93315, t93318, t93320, t93322, t93324, t93326)
}
