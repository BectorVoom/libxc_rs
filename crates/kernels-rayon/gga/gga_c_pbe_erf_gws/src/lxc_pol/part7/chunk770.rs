//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 770/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk770(t2345: f64, t6206: f64, t875: f64, t2119: f64, t2387: f64, t2124: f64, t2145: f64, t2150: f64, t3074: f64, t6252: f64, t2112: f64, t5: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6316 = t2345 * t6206 * t875;
    let t6319 = t2387 * t2119;
    let t6321 = t6319 * t2124 / 48.0_f64;
    let t6322 = t2387 * t2145;
    let t6324 = t6322 * t2150 / 16.0_f64;
    let t6325 = t3074 * t6252;
    let t6326 = t5 * t2112;
    (t6316, t6319, t6321, t6322, t6324, t6325, t6326)
}
