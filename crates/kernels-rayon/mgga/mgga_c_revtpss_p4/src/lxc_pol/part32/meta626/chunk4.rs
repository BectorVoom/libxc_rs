//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1994/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1994(t1353: f64, t2106: f64, t101970: f64, t28154: f64, t101782: f64, t101783: f64, t101790: f64, t101793: f64, t101811: f64, t101820: f64, t108941: f64, t1923: f64, t2047: f64, t28093: f64, t28635: f64, t30543: f64, t6954: f64, t7702: f64, t7964: f64, t95246: f64) -> (f64, f64) {
    let t109874 = t1353 * t2106;
    let t109892 = t28154 * t101970;
    let t109895 = t101782 - 880.0_f64 / 27.0_f64 * t101783 + t101790 - 352.0_f64 / 27.0_f64 * t101793 + t101811 + 2.0_f64 / 3.0_f64 * t28093 * t7964 + 2.0_f64 / 3.0_f64 * t7702 * t28635 + t6954 * t30543 / 3.0_f64 + t1923 * t2047 * t108941 / 3.0_f64 - 160.0_f64 / 9.0_f64 * t109892 + 88.0_f64 / 27.0_f64 * t95246 + t101820;
    (t109874, t109895)
}
