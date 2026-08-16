//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 453/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk453(t357: f64, t905: f64, t1065: f64, t126: f64, t1086: f64, t994: f64, t3090: f64, t373: f64, t66: f64, t828: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3094 = t357 * t905;
    let t3109 = t126 * t1065;
    let t3114 = t994 * t1086;
    let t3115 = t3114 * t3090;
    let t3116 = t66 * t373;
    let t3117 = t828 * t3116;
    (t3094, t3109, t3114, t3115, t3116, t3117)
}
