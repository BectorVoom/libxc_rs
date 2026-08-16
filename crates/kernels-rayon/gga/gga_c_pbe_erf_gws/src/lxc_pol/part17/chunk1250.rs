//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1250/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1250(t13781: f64, t14582: f64, t3972: f64, t9550: f64, t14592: f64, t50994: f64, t14657: f64, t6797: f64, t14136: f64, t8690: f64, t2112: f64, t2306: f64, t3975: f64, t9385: f64) -> (f64, f64, f64, f64, f64) {
    let t53351 = t3972 * t13781 * t14582 * t9550;
    let t53353 = t50994 * t14592;
    let t53354 = 7.0_f64 / 288.0_f64 * t53353;
    let t53355 = t14657 * t6797;
    let t53357 = t14136 * t8690;
    let t53362 = t3972 * t3975 * t9385 * t2306 * t2112;
    (t53351, t53354, t53355, t53357, t53362)
}
