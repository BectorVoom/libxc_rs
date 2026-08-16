//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 903/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk903(t1683: f64, t1750: f64, t1820: f64, t1885: f64, t5273: f64, t562: f64, t597: f64, t16978: f64, t639: f64, t642: f64, t643: f64, t1627: f64, t5464: f64) -> (f64, f64, f64, f64) {
    let t17057 = t1750 * t1683;
    let t17058 = 16.0_f64 / 15.0_f64 * t17057;
    let t17063 = 16.0_f64 / 15.0_f64 * t1820 * t1885 * t597 * t5273 * t562;
    let t17067 = 4.0_f64 / 45.0_f64 * t639 * t642 * t643 * t16978;
    let t17068 = t1627 * t5464;
    (t17058, t17063, t17067, t17068)
}
