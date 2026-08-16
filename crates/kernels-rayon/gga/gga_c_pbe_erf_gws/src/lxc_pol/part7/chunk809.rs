//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 809/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk809(t2206: f64, t2216: f64, t4379: f64, t858: f64, t886: f64, t884: f64, t2170: f64, t6177: f64, t6287: f64, t3138: f64, t346: f64, t4408: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6691 = t2206 * t2216;
    let t6692 = 7.0_f64 / 48.0_f64 * t6691;
    let t6694 = t886 * t858 * t4379;
    let t6696 = t884 * t6694 / 48.0_f64;
    let t6698 = t2170 * t6177 * t6287;
    let t6700 = t3138 * t6698 / 8.0_f64;
    let t6701 = t4408 * t346;
    (t6692, t6694, t6696, t6698, t6700, t6701)
}
