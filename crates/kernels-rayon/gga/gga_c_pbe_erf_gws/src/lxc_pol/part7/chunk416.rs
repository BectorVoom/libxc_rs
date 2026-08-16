//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 416/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk416(t1750: f64, t221: f64, t174: f64, t177: f64, t332: f64, t395: f64, t574: f64, t56: f64, t589: f64) -> (f64, f64, f64, f64, f64) {
    let t1752 = 2.0_f64 / 15.0_f64 * t1750 * t221;
    let t1754 = t174 * t332 * t177;
    let t1755 = 0.25188888888888888889e-2_f64 * t1754;
    let t1756 = t395 * t574;
    let t1758 = t56 * t589;
    (t1752, t1754, t1755, t1756, t1758)
}
