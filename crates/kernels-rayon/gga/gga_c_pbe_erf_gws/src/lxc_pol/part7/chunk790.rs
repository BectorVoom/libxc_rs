//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 790/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk790(t2170: f64, t2171: f64, t6177: f64, t2168: f64, t2122: f64, t337: f64, t810: f64, t2147: f64, t2120: f64, t2133: f64, t2387: f64, t2138: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6530 = t2170 * t6177 * t2171;
    let t6532 = t2168 * t6530 / 16.0_f64;
    let t6534 = t337 * t2122 * t810;
    let t6535 = t2147 * t6534;
    let t6537 = t2120 * t6535 / 16.0_f64;
    let t6538 = t2387 * t2133;
    let t6540 = t6538 * t2138 / 32.0_f64;
    (t6530, t6532, t6534, t6535, t6537, t6538, t6540)
}
