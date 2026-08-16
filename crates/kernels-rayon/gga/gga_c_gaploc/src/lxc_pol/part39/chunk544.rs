//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 544/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk544(t1: f64, t3085: f64, t106: f64, t192: f64, t3152: f64, t528: f64, t3148: f64, t1564: f64, t3116: f64, t475: f64, t1445: f64, t4529: f64) -> (f64, f64, f64, f64, f64) {
    let t9391 = t3085 * t1;
    let t9392 = t9391 * t106;
    let t9393 = t9392 * t192;
    let t9396 = t528 * t3152;
    let t9399 = t528 * t3148;
    let t9402 = t1564 * t3116;
    let t9403 = t9402 * t475;
    let t9404 = t1445 * t9403;
    let t9407 = t4529 * t3085;
    (t9393, t9396, t9399, t9404, t9407)
}
