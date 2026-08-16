//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1150/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1150(t2147: f64, t337: f64, t6568: f64, t810: f64, t6567: f64, t2319: f64, t6474: f64, t2189: f64, t343: f64, t814: f64, t3065: f64, t858: f64) -> (f64, f64, f64, f64) {
    let t20591 = t2147 * t337 * t6568 * t810;
    let t20593 = t6567 * t20591 / 6.0_f64;
    let t20594 = t2319 * t6474;
    let t20597 = t814 * t2189 * t343;
    let t20599 = t3065 * t858 * t20597;
    (t20593, t20594, t20597, t20599)
}
