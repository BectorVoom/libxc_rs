//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 966/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk966(t2615: f64, t2632: f64, t3443: f64, t597: f64, t562: f64, t1885: f64, t1820: f64, t3534: f64, t5018: f64, t1017: f64, t7468: f64, t7467: f64) -> (f64, f64, f64, f64) {
    let t10907 = 8.0_f64 / 15.0_f64 * t2615 * t2632;
    let t10908 = t597 * t3443;
    let t10909 = t10908 * t562;
    let t10910 = t1885 * t10909;
    let t10912 = 4.0_f64 / 15.0_f64 * t1820 * t10910;
    let t10913 = t5018 * t3534;
    let t10914 = t1820 * t10913;
    let t10915 = 16.0_f64 / 45.0_f64 * t10914;
    let t10916 = t7468 * t1017;
    let t10917 = t7467 * t10916;
    (t10907, t10912, t10915, t10917)
}
