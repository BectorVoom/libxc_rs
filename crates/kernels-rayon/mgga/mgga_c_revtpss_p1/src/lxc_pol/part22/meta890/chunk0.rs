//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3078/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3078(t378: f64, t53014: f64, t1072: f64, t994: f64, t3046: f64, t379: f64, t11213: f64, t1678: f64, t16237: f64, t342: f64, t11120: f64, t1695: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t53015 = t53014 * t378;
    let t53027 = t994 * t1072;
    let t53034 = t3046 * t379;
    let t53058 = t11213 * t1678;
    let t53093 = t342 * t16237;
    let t53108 = t11120 * t1695;
    (t53015, t53027, t53034, t53058, t53093, t53108)
}
