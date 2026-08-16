//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1192/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1192(t6484: f64, t6526: f64, t2127: f64, t6619: f64, t850: f64, t860: f64, t346: f64, t6110: f64, t822: f64, t2150: f64, t2083: f64, t6104: f64) -> (f64, f64, f64, f64) {
    let t21182 = t6484 * t6526;
    let t21183 = 7.0_f64 / 4.0_f64 * t21182;
    let t21187 = t850 * t6619 * t2127 * t860 / 32.0_f64;
    let t21188 = t6110 * t346;
    let t21189 = t822 * t21188;
    let t21191 = t21189 * t2150 / 12.0_f64;
    let t21196 = t6104 * t2083;
    (t21183, t21187, t21191, t21196)
}
