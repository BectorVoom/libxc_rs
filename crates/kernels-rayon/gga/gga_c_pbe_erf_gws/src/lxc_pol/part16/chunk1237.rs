//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1237/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1237(t14797: f64, t3989: f64, t3990: f64, t9321: f64, t13781: f64, t14582: f64, t3972: f64, t9380: f64, t9550: f64, t14592: f64, t50994: f64, t14657: f64, t6797: f64) -> (f64, f64, f64, f64, f64) {
    let t53338 = t3989 * t3990 * t14797 * t9321;
    let t53346 = t3972 * t13781 * t14582 * t9380;
    let t53351 = t3972 * t13781 * t14582 * t9550;
    let t53353 = t50994 * t14592;
    let t53355 = t14657 * t6797;
    (t53338, t53346, t53351, t53353, t53355)
}
