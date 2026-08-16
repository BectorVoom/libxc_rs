//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 470/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk470(t1923: f64, t248: f64, t256: f64, t528: f64, t713: f64, t1881: f64, t1884: f64, t1890: f64, t1895: f64, t1900: f64, t1902: f64, t1905: f64, t1907: f64, t1911: f64, t1915: f64, t1920: f64) -> (f64, f64, f64, f64) {
    let t1924 = t248 * t1923;
    let t1926 = t1924 * t256 / 3.0_f64;
    let t1928 = 0.33245444444444444444e-1_f64 * t528 * t713;
    let t1929 = t1881 + t1884 - t1890 - t1895 - t1900 + t1902 - t1905 + t1907 * t256 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t1911 + 0.12155555555555555555e0_f64 * t1915 + t1920 + t1926 + t1928;
    (t1924, t1926, t1928, t1929)
}
