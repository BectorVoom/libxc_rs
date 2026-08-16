//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 479/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk479(t226: f64, t678: f64, t230: f64, t666: f64, t1826: f64, t1831: f64, t1870: f64, t1874: f64, t1876: f64, t1881: f64, t1884: f64, t1890: f64, t1895: f64, t1900: f64) -> (f64, f64, f64) {
    let t2014 = 8.0_f64 / 3.0_f64 * t226 * t678;
    let t2015 = t666 * t230;
    let t2017 = t1826 - t1831 - t1870 + t1874 - t1876 + t1881 + t1884 + t2014 + 8.0_f64 / 3.0_f64 * t2015 - t1890 - t1895 - t1900;
    (t2014, t2015, t2017)
}
