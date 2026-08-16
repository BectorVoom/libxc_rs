//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 843/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk843(t2344: f64, t904: f64, t4383: f64, t6158: f64, t2157: f64, t3222: f64, t1185: f64, t346: f64, t825: f64, t38: f64, t368: f64, t4340: f64, param_a_c: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9665 = t2344 * t904;
    let t11374 = t6158 * t4383;
    let t11540 = t2157 * param_a_c;
    let t11541 = t11540 * t3222;
    let t12076 = t346 * t825 * t1185;
    let t15651 = t38 * t38;
    let t15652 = 1.0_f64 / t15651;
    let t16191 = t368 * t368;
    let t16192 = 1.0_f64 / t16191;
    let t16329 = 0.12654485932329694421e2_f64 * t4340;
    (t9665, t11374, t11541, t12076, t15651, t15652, t16192, t16329)
}
