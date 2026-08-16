//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 467/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk467(t562: f64, t597: f64, t610: f64, t1885: f64, t1820: f64, t1697: f64, t219: f64, t1413: f64, t642: f64, t639: f64, t1764: f64, t197: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1887 = t597 * t562 * t610;
    let t1888 = t1885 * t1887;
    let t1890 = 8.0_f64 / 15.0_f64 * t1820 * t1888;
    let t1891 = t219 * t1697;
    let t1892 = t1891 * t1413;
    let t1893 = t642 * t1892;
    let t1895 = 8.0_f64 / 45.0_f64 * t639 * t1893;
    let t1896 = t197 * t1764;
    (t1887, t1888, t1890, t1892, t1893, t1895, t1896)
}
