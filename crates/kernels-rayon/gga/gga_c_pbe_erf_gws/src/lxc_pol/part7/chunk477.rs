//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 477/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk477(t2113: f64, t850: f64, t852: f64, t860: f64, t2083: f64, t274: f64) -> (f64, f64, f64) {
    let t2115 = t850 * t2113 * t852;
    let t2117 = t2115 * t860 / 96.0_f64;
    let t2118 = t2083 * t274;
    (t2115, t2117, t2118)
}
