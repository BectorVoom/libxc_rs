//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1058/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1058(t13249: f64, t6402: f64, t3123: f64, t37138: f64, t12041: f64, t36666: f64, t13342: f64, t6416: f64, t13124: f64, t19561: f64, t13446: f64, t2206: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t45755 = t6402 * t13249;
    let t45767 = t3123 * t37138;
    let t45771 = t12041 * t36666;
    let t45793 = t6416 * t13342;
    let t45805 = t13124 * t19561;
    let t45821 = t2206 * t13446;
    (t45755, t45767, t45771, t45793, t45805, t45821)
}
