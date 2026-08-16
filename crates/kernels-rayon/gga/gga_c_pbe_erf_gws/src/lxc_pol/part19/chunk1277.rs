//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1277/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1277(t11443: f64, t13917: f64, t53138: f64, t14583: f64, t53496: f64, t53841: f64, t53923: f64, t9942: f64, t11354: f64, t14797: f64, t3989: f64, t3990: f64) -> (f64, f64, f64, f64) {
    let t56206 = t13917 * t53138 * t11443;
    let t56209 = t13917 * t53496 * t14583;
    let t56236 = t53923 * t53841 * t9942;
    let t56240 = t3989 * t3990 * t14797 * t11354;
    (t56206, t56209, t56236, t56240)
}
