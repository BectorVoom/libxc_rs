//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1247/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1247(t159: f64, t3181: f64, t2851: f64, t631: f64, t45: f64, t1071: f64, t3057: f64, t3259: f64, t994: f64, t342: f64, t992: f64, t338: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11142 = t159 * t3181;
    let t11144 = 1.0_f64 / t2851 / t631;
    let t11149 = t2851 * t45;
    let t11150 = 1.0_f64 / t11149;
    let t11187 = t3057 * t1071;
    let t11190 = t994 * t3259;
    let t11195 = t342 * t3259;
    let t11198 = t992 * t992;
    let t11199 = 1.0_f64 / t11198;
    let t11200 = t338 * t11199;
    (t11142, t11144, t11150, t11187, t11190, t11195, t11200)
}
