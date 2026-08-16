//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 966/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk966(t10898: f64, t184: f64, t997: f64, t7171: f64, t5465: f64, t10888: f64, t10890: f64, t10894: f64, t10895: f64, t10897: f64, t5418: f64, t5423: f64, t5429: f64, t5430: f64, t5433: f64, t5436: f64, t5437: f64, t5443: f64, t7775: f64) -> (f64, f64, f64, f64) {
    let t10899 = t10898 * t184;
    let t10901 = 8.0_f64 / 15.0_f64 * t10899 * t997;
    let t10903 = 8.0_f64 / 15.0_f64 * t7171 * t997;
    let t10904 = 8.0_f64 / 405.0_f64 * t5465;
    let t10905 = 0.12155555555555555555e0_f64 * t5418 + t5423 + t5429 + 4.0_f64 / 9.0_f64 * t5430 + t5433 + t5436 - 2.0_f64 / 27.0_f64 * t5437 - t5443 + t10888 + t10890 + t10894 + t7775 - t10895 - t10897 + t10901 + t10903 - t10904;
    (t10901, t10903, t10904, t10905)
}
