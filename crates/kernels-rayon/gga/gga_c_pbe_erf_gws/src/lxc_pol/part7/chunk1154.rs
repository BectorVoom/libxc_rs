//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1154/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1154(t2387: f64, t6566: f64, t6570: f64, t2105: f64, t814: f64, t6587: f64, t899: f64, t912: f64, t918: f64, t6198: f64, t6416: f64, t6183: f64, t6569: f64) -> (f64, f64, f64, f64, f64) {
    let t20638 = t2387 * t6566 * t6570 / 16.0_f64;
    let t20640 = t2105 * t814;
    let t20646 = t899 * t912 * t6587;
    let t20647 = t20646 * t918;
    let t20649 = t6416 * t6198;
    let t20651 = t6183 * t6569;
    (t20638, t20640, t20647, t20649, t20651)
}
