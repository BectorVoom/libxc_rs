//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 955/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk955(t1022: f64, t5109: f64, t108: f64, t267: f64, t2789: f64, t1917: f64, t2519: f64, t1062: f64, t5385: f64, t1045: f64, t2735: f64, t211: f64) -> (f64, f64, f64, f64, f64) {
    let t25081 = t5109 * t1022;
    let t25208 = t2789 * t108 * t267;
    let t25230 = t2519 * t1917;
    let t25349 = t1062 * t5385;
    let t25353 = t2735 * t1045;
    let t25354 = t211 * t25353;
    (t25081, t25208, t25230, t25349, t25354)
}
