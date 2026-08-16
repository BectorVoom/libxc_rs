//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1248/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1248(t14602: f64, t51666: f64, t3959: f64, t9704: f64, t3965: f64, t9323: f64, t13917: f64, t14424: f64, t9551: f64, t14415: f64, t51563: f64, t13776: f64, t36865: f64, t3975: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t53597 = t51666 * t14602;
    let t53599 = t3959 * t9704;
    let t53601 = t3965 * t9323;
    let t53623 = t13917 * t14424 * t9551;
    let t53625 = t51563 * t14415;
    let t53631 = t13776 * t3975 * t36865;
    (t53597, t53599, t53601, t53623, t53625, t53631)
}
