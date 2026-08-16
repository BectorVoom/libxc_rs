//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 566/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk566(t2157: f64, t874: f64, t19: f64, t369: f64, t332: f64, t329: f64, t343: f64, t274: f64, t851: f64, t253: f64, t903: f64, param_a_c: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3140 = t2157 * t874;
    let t3205 = t369 * t19;
    let t3206 = t332 * t3205;
    let t3207 = t329 * t3206;
    let t3221 = t343 * param_a_c;
    let t3222 = t851 * t274;
    let t3223 = t3221 * t3222;
    let t3235 = t903 * t253;
    (t3140, t3205, t3206, t3207, t3222, t3223, t3235)
}
