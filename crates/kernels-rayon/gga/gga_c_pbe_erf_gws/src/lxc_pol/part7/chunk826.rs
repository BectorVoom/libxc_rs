//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 826/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk826(t6472: f64, t825: f64, t6800: f64, t2239: f64, t2246: f64, t2409: f64, t6449: f64, t831: f64, t329: f64, t332: f64, t931: f64, t376: f64, t6385: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6801 = t6472 * t825;
    let t6802 = t6800 * t6801;
    let t6805 = t2246 * t2239;
    let t6810 = t2409 * t831 * t6449;
    let t6816 = t329 * t332 * t931;
    let t6817 = t376 * t6385;
    (t6801, t6802, t6805, t6810, t6816, t6817)
}
