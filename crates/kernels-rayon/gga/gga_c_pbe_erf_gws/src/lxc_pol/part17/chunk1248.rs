//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1248/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1248(t14797: f64, t3989: f64, t3990: f64, t8647: f64, t14669: f64, t9270: f64, t14448: f64, t4414: f64, t13917: f64, t13919: f64, t9433: f64, t13859: f64, t9218: f64) -> (f64, f64, f64, f64, f64) {
    let t53299 = t3989 * t3990 * t14797 * t8647;
    let t53302 = 7.0_f64 / 72.0_f64 * t9270 * t14669;
    let t53308 = 7.0_f64 / 72.0_f64 * t4414 * t14448;
    let t53323 = t13917 * t13919 * t9433;
    let t53327 = t13859 * t3990 * t14797 * t9218;
    (t53299, t53302, t53308, t53323, t53327)
}
