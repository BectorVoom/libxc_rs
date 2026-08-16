//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 380/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk380(t1515: f64, t1557: f64, t1243: f64, t502: f64, t505: f64, t95: f64) -> (f64, f64, f64) {
    let t1558 = t1557 * t1515;
    let t1561 = 0.32645333333333333333e0_f64 * t502 * t1243;
    let t1563 = 1.0_f64 / t505 / t95;
    (t1558, t1561, t1563)
}
