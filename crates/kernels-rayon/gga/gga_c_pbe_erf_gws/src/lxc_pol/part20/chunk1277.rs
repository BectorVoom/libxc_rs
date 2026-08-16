//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1277/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1277(t14657: f64, t53250: f64, t1134: f64, t13776: f64, t3060: f64, t50956: f64, t13859: f64, t52926: f64, t9942: f64, t1109: f64, t1192: f64, t11443: f64, t13917: f64, t53138: f64) -> (f64, f64, f64, f64, f64) {
    let t56190 = t14657 * t53250;
    let t56194 = t13776 * t50956 * t1134 * t3060;
    let t56197 = t13859 * t52926 * t9942;
    let t56199 = t1192 * t1109;
    let t56206 = t13917 * t53138 * t11443;
    (t56190, t56194, t56197, t56199, t56206)
}
