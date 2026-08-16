//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1198/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1198(t3039: f64, t4384: f64, t6792: f64, t1114: f64, t19776: f64, t2200: f64, t857: f64, t329: f64, t6126: f64, t891: f64, t19658: f64, t2409: f64, t3205: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t22343 = t3039 * t4384;
    let t22379 = t3039 * t6792;
    let t22493 = t1114 * t19776;
    let t22508 = t2200 * t857;
    let t22509 = t329 * t22508;
    let t22534 = t891 * t6126;
    let t26604 = t1114 * t19658;
    let t26617 = t3205 * t2409;
    (t22343, t22379, t22493, t22509, t22534, t26604, t26617)
}
