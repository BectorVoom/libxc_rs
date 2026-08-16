//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1156/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1156(t2100: f64, t2121: f64, t2122: f64, t337: f64, t6567: f64, t814: f64, t9465: f64, t6401: f64, t6684: f64, t6688: f64, t2189: f64, t343: f64, t816: f64) -> (f64, f64, f64, f64) {
    let t20667 = t2121 * t337 * t2122 * t2100;
    let t20669 = t6567 * t20667 / 16.0_f64;
    let t20670 = t9465 * t814;
    let t20675 = t6684 * t6401;
    let t20676 = t20675 * t6688;
    let t20682 = t816 * t2189 * t343;
    (t20669, t20670, t20676, t20682)
}
