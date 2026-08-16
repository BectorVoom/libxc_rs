//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 946/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk946(t1333: f64, t2515: f64, t4847: f64, t6967: f64, t4844: f64, t4838: f64, t2840: f64, t4805: f64, t1114: f64, t19776: f64, t409: f64, t7996: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t22063 = t1333 * t2515;
    let t22066 = t6967 * t4847;
    let t22068 = t6967 * t4844;
    let t22070 = t6967 * t4838;
    let t22084 = t2840 * t4805;
    let t22493 = t1114 * t19776;
    let t22590 = t409 * t7996;
    (t22063, t22066, t22068, t22070, t22084, t22493, t22590)
}
