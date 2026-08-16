//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 848/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk848(t1378: f64, t535: f64, t6056: f64, t922: f64, t1478: f64, t1480: f64, t4579: f64, t4585: f64, t6054: f64, t147: f64, t2331: f64, t1533: f64) -> (f64, f64, f64, f64, f64) {
    let t16411 = t922 * t535 * t1378 * t6056;
    let t16415 = 0.10931146159029059066e-3_f64 * t1478 * t4579 * t1480;
    let t16418 = 0.18276876377896586758e-4_f64 * t6054 * t4585 * t6056;
    let t16422 = 0.67015213385620818113e-4_f64 * t2331 * t147 * t1378 * t6056;
    let t16423 = t1533 * t1533;
    (t16411, t16415, t16418, t16422, t16423)
}
