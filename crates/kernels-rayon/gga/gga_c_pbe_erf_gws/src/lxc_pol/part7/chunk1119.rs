//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1119/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1119(t6729: f64, t894: f64, t2083: f64, t2108: f64, t825: f64, t2169: f64, t2200: f64, t329: f64, t2412: f64, t2239: f64, t4442: f64, t4414: f64, t6828: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t20081 = t6729 * t894;
    let t20085 = t2083 * t2108;
    let t20086 = t20085 * t825;
    let t20091 = t329 * t2200 * t2169;
    let t20092 = t20091 * t2412;
    let t20106 = t4442 * t2239;
    let t20108 = t4414 * t6828;
    (t20081, t20085, t20086, t20092, t20106, t20108)
}
