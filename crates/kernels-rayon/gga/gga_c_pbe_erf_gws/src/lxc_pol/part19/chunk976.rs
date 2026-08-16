//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 976/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk976(t11028: f64, t1621: f64, t639: f64, t3478: f64, t586: f64, t645: f64, t11004: f64, t11009: f64, t11014: f64, t11016: f64, t11018: f64, t11021: f64, t11024: f64, t11027: f64, t7852: f64, t7870: f64, t7873: f64, t7876: f64, t7880: f64, t7890: f64, t7905: f64) -> (f64, f64, f64) {
    let t11029 = t1621 * t11028;
    let t11031 = 8.0_f64 / 15.0_f64 * t639 * t11029;
    let t11032 = t3478 * t586;
    let t11034 = 4.0_f64 / 45.0_f64 * t11032 * t645;
    let t11035 = t11004 + t7852 + t7870 - t7873 - t7876 + t7880 + t7890 + t11009 + t11014 + t11016 - t7905 - t11018 + t11021 - t11024 - t11027 + t11031 + t11034;
    (t11031, t11034, t11035)
}
