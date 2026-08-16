//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 666/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk666(t5304: f64, t593: f64, t1406: f64, t597: f64, t610: f64, t1885: f64, t1820: f64, t1878: f64, t586: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5306 = 8.0_f64 / 15.0_f64 * t5304 * t593;
    let t5307 = t597 * t1406;
    let t5308 = t5307 * t610;
    let t5309 = t1885 * t5308;
    let t5311 = 4.0_f64 / 5.0_f64 * t1820 * t5309;
    let t5312 = t1878 * t586;
    (t5306, t5307, t5308, t5309, t5311, t5312)
}
