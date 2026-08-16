//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 696/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk696(t1: f64, t3360: f64, t467: f64, t1167: f64, t2429: f64, t2053: f64, t3928: f64, t3342: f64, t4351: f64, t3351: f64, t4366: f64, t2358: f64, t3916: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9762 = t3360 * t1;
    let t9763 = t9762 * t467;
    let t9766 = t2429 * t1167;
    let t9772 = t3928 * t2053;
    let t9778 = t4351 * t3342;
    let t9793 = t4366 * t3351;
    let t9815 = t3916 * t2358;
    (t9762, t9763, t9766, t9772, t9778, t9793, t9815)
}
