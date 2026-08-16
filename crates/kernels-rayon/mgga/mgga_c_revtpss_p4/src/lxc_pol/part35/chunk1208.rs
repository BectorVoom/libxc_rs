//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1208/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1208(t109892: f64, t109983: f64, t109985: f64, t109988: f64, t109990: f64, t110008: f64, t110010: f64, t110014: f64, t114343: f64, t1923: f64, t2047: f64, t29513: f64, t30543: f64, t7702: f64, t7964: f64, t95253: f64) -> f64 {
    let t115305 = t29513 * t7964 + t7702 * t30543 + t1923 * t2047 * t114343 / 3.0_f64 - 160.0_f64 / 3.0_f64 * t109892 - t95253 - 8.0_f64 / 3.0_f64 * t109983 - 16.0_f64 / 3.0_f64 * t109985 - 8.0_f64 / 3.0_f64 * t109988 + 16.0_f64 / 3.0_f64 * t109990 + 80.0_f64 / 3.0_f64 * t110008 + 32.0_f64 / 3.0_f64 * t110010 + 80.0_f64 / 3.0_f64 * t110014;
    t115305
}
