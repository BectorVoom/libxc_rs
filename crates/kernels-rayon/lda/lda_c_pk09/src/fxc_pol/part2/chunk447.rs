//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 447/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk447(t2378: f64, t83: f64, t89: f64, t1047: f64, t1052: f64, t106: f64, t1065: f64, t1076: f64, t1101: f64, t115: f64, t2155: f64, t2210: f64, t2214: f64, t2305: f64, t2337: f64, t2341: f64, t2355: f64, t2363: f64, t90: f64, t98: f64, t993: f64, t994: f64) -> (f64, f64, f64) {
    let t2379 = t83 * t2378;
    let t2380 = t2379 * t89;
    let t2385 = 0.14975624337724558_f64 * t2155 + t2337 * t98 / 6.0_f64 + t115 * t2341 / 6.0_f64 + t2305 * t1047 / 36.0_f64 - t993 - t994 + t1052 * t2210 / 6.0_f64 + t1052 * t2214 / 6.0_f64 - t2355 * t98 / 6.0_f64 + t1076 * t2210 / 6.0_f64 + t1076 * t2214 / 6.0_f64 - t2363 * t98 / 6.0_f64 - t1101 * t2210 / 6.0_f64 - t1101 * t2214 / 6.0_f64 - t106 * t2341 / 6.0_f64 - t2380 * t98 / 6.0_f64 + t1065 - t90 * t2341 / 6.0_f64;
    (t2379, t2380, t2385)
}
