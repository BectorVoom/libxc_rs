//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 444/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk444(t1903: f64, t252: f64, t1354: f64, t247: f64, t251: f64, t707: f64, t719: f64, t256: f64, t19: f64, t535: f64, t336: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1905 = 2.0_f64 / 27.0_f64 * t252 * t1903;
    let t1906 = t1354 * t247;
    let t1907 = t1906 * t251;
    let t1910 = t707 * t719;
    let t1911 = t1910 * t256;
    let t1913 = t535 * t19;
    let t1914 = t1913 * t336;
    (t1905, t1906, t1907, t1910, t1911, t1913, t1914)
}
