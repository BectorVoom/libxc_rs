//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 823/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk823(t2206: f64, t2216: f64, t346: f64, t4408: f64, t2100: f64, t5: f64, t337: f64, t2121: f64, t2271: f64, t822: f64, t2273: f64, t2319: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6691 = t2206 * t2216;
    let t6701 = t4408 * t346;
    let t6705 = t5 * t2100;
    let t6706 = t337 * t6705;
    let t6707 = t2121 * t6706;
    let t6710 = t2271 * t346;
    let t6711 = t822 * t6710;
    let t6714 = t2319 * t2273;
    (t6691, t6701, t6706, t6707, t6710, t6711, t6714)
}
