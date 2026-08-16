//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1233/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1233(t111405: f64, t111408: f64, t111410: f64, t111411: f64, t111412: f64, t111415: f64, t116008: f64, t116023: f64, t1458: f64, t1914: f64, t1921: f64, t2111: f64, t2118: f64, t25049: f64, t25072: f64, t3: f64, t30627: f64, t30663: f64, t575: f64, t6937: f64, t6951: f64, t8114: f64, t8130: f64) -> f64 {
    let tv4rho3sigma10 = t116008 * t3 * t575 + t116023 * t1458 + 3.0_f64 * t1914 * t30663 + 3.0_f64 * t1921 * t30627 + t2111 * t25072 + t2118 * t25049 + 3.0_f64 * t6937 * t8130 + 3.0_f64 * t6951 * t8114 + 3.0_f64 * t111405 + 6.0_f64 * t111408 + 3.0_f64 * t111410 + 3.0_f64 * t111411 + 6.0_f64 * t111412 + 3.0_f64 * t111415;
    tv4rho3sigma10
}
