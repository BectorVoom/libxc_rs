//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 785/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk785(t6471: f64, t6472: f64, t905: f64, t2308: f64, t2319: f64, t1477: f64, t855: f64, t863: f64, t888: f64, t838: f64, t864: f64) -> (f64, f64, f64, f64, f64) {
    let t6473 = t6471 * t6472;
    let t6474 = t905 * t6473;
    let t6477 = t2319 * t2308;
    let t6480 = t863 * t855 * t1477;
    let t6481 = t6480 * t888;
    let t6482 = 35.0_f64 / 72.0_f64 * t6481;
    let t6484 = t863 * t864 * t838;
    (t6474, t6477, t6480, t6482, t6484)
}
