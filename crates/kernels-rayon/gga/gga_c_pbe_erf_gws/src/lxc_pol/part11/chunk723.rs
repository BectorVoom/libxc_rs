//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 723/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk723(t1651: f64, t3526: f64, t587: f64, t3390: f64, t626: f64, t3399: f64, t583: f64, t1802: f64, t3443: f64, t5109: f64, t3380: f64, t700: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11037 = t1651 * t3526;
    let t11038 = t587 * t11037;
    let t11054 = t3390 * t626;
    let t11065 = t3399 * t583;
    let t11110 = t1802 * t3443;
    let t11136 = t5109 * t3390;
    let t11157 = t3380 * t700;
    (t11037, t11038, t11054, t11065, t11110, t11136, t11157)
}
