//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1218/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1218(t2108: f64, t339: f64, t2080: f64, t2084: f64, t860: f64, t2142: f64, t6493: f64, t20133: f64, t326: f64, t6094: f64, t19561: f64, t20134: f64) -> (f64, f64, f64) {
    let t21610 = t2108 * t339;
    let t21614 = t2080 * t2084 * t21610 * t860 / 32.0_f64;
    let t21615 = t6493 * t2142;
    let t21616 = 7.0_f64 / 72.0_f64 * t21615;
    let t21621 = t326 * t20133;
    let t21623 = t6094 * t339;
    let t21627 = t21621 * t20134 * t19561 * t21623 * t860 / 96.0_f64;
    (t21614, t21616, t21627)
}
