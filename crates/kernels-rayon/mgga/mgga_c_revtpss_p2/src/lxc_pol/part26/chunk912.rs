//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 912/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk912(t3154: f64, t905: f64, t606: f64, t11659: f64, t3092: f64, t3095: f64, t1052: f64, t360: f64, t3089: f64, t1087: f64, t3090: f64, t3278: f64) -> (f64, f64, f64, f64, f64) {
    let t11660 = t3154 * t905;
    let t11661 = t11660 * t606;
    let t11662 = t11659 * t11661;
    let t11663 = t3092 * t11662;
    let t11666 = t11659 * t3095;
    let t11667 = t3092 * t11666;
    let t11670 = t360 * t1052;
    let t11671 = t11670 * t3089;
    let t11672 = t1087 * t11671;
    let t11675 = t3278 * t3090;
    (t11663, t11667, t11671, t11672, t11675)
}
