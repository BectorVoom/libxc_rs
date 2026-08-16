//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1327/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1327(t14093: f64, t57222: f64, t11439: f64, t54047: f64, t11746: f64, t51351: f64, t11431: f64, t51306: f64, t11854: f64, t14031: f64, t11860: f64, t4028: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t57223 = t57222 * t14093;
    let t57225 = t54047 * t11439;
    let t57227 = t51351 * t11746;
    let t57229 = t51306 * t11431;
    let t57231 = t14031 * t11854;
    let t57233 = t4028 * t11860;
    (t57223, t57225, t57227, t57229, t57231, t57233)
}
