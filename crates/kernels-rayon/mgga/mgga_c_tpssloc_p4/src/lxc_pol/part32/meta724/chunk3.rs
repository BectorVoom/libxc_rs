//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2321/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2321(t1222: f64, t29601: f64, t104107: f64, t104111: f64, t104120: f64, t104124: f64, t104126: f64, t1232: f64, t18383: f64, t18965: f64, t2136: f64, t24736: f64, t24741: f64, t25588: f64, t29625: f64, t6207: f64, t7316: f64, t8027: f64, t86191: f64, t86327: f64, t95370: f64) -> f64 {
    let t104128 = t29601 * t1222;
    let t104134 = t86191 - t24736 * t6207 / 2304.0_f64 - 19.0_f64 / 1296.0_f64 * t104107 * t1232 + t95370 + 0.16149102437656156342e-2_f64 * t104111 + 0.16149102437656156342e-2_f64 * t8027 * t25588 * t2136 + 0.10093189023535097714e-3_f64 * t7316 * t29625 + 0.20186378047070195428e-3_f64 * t104120 - 0.10093189023535097714e-3_f64 * t104124 - t104126 / 216.0_f64 + 19.0_f64 / 1296.0_f64 * t104128 - t24741 * t18383 / 2304.0_f64 + t86327 * t18965 / 2304.0_f64;
    t104134
}
