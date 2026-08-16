//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1256/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1256(t20154: f64, t2376: f64, t4207: f64, t814: f64, t14327: f64, t3083: f64, t53353: f64, t27047: f64, t3067: f64, t4216: f64, t1205: f64, t26654: f64) -> (f64, f64, f64, f64, f64) {
    let t55110 = t20154 * t2376 * t4207 * t814;
    let t55114 = 7.0_f64 / 144.0_f64 * t3083 * t14327;
    let t55117 = 7.0_f64 / 144.0_f64 * t53353;
    let t55137 = t27047 * t3067 * t4216 * t814;
    let t55140 = t26654 * t1205;
    (t55110, t55114, t55117, t55137, t55140)
}
