//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1305/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1305(t11625: f64, t14007: f64, t11521: f64, t14498: f64, t11930: f64, t14015: f64, t11750: f64, t51351: f64, t11444: f64, t11938: f64, t11427: f64, t51306: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t56894 = t14007 * t11625;
    let t56896 = t14498 * t11521;
    let t56898 = t14015 * t11930;
    let t56901 = t51351 * t11750;
    let t56903 = t51351 * t11444;
    let t56905 = t14498 * t11938;
    let t56910 = t51306 * t11427;
    (t56894, t56896, t56898, t56901, t56903, t56905, t56910)
}
