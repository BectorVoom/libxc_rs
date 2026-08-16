//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 468/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk468(t1403: f64, t1896: f64, t590: f64, t587: f64, t720: f64, t723: f64, t156: f64, t254: f64, t252: f64, t1354: f64, t247: f64, t251: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1897 = t1896 * t1403;
    let t1898 = t590 * t1897;
    let t1900 = 8.0_f64 / 45.0_f64 * t587 * t1898;
    let t1902 = 4.0_f64 / 9.0_f64 * t720 * t723;
    let t1903 = t254 * t156;
    let t1905 = 2.0_f64 / 27.0_f64 * t252 * t1903;
    let t1906 = t1354 * t247;
    let t1907 = t1906 * t251;
    (t1897, t1898, t1900, t1902, t1903, t1905, t1906, t1907)
}
