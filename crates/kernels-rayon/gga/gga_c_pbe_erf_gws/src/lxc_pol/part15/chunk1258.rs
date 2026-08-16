//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1258/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1258(t14121: f64, t9215: f64, t13917: f64, t14423: f64, t3223: f64, t361: f64, t874: f64, t14724: f64, t343: f64, t13783: f64, t14809: f64, t4414: f64) -> (f64, f64, f64, f64) {
    let t53487 = t14121 * t9215;
    let t53493 = t13917 * t361 * t14423 * t874 * t3223;
    let t53496 = t361 * t14724 * t343;
    let t53498 = t13917 * t53496 * t13783;
    let t53503 = 7.0_f64 / 72.0_f64 * t4414 * t14809;
    (t53487, t53493, t53498, t53503)
}
