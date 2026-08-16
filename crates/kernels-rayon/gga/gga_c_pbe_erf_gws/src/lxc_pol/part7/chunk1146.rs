//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1146/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1146(t2305: f64, t904: f64, t2257: f64, t4408: f64, t20251: f64, t2118: f64, t6416: f64, t6575: f64, t20499: f64, t20500: f64, t20505: f64, t20511: f64, t20514: f64, t20516: f64, t20522: f64, t2292: f64, t6275: f64, t6276: f64, t6637: f64, t6639: f64, t9505: f64, t9637: f64) -> (f64, f64) {
    let t20527 = t904 * t2305;
    let t20528 = t4408 * t2257;
    let t20532 = t2118 * t20251;
    let t20536 = t6416 * t6575;
    let t20538 = t20499 + t6637 * t20500 * t6639 / 96.0_f64 + 3.0_f64 / 64.0_f64 * t9637 * t6276 * t20505 + t20511 + t20514 - 3.0_f64 / 128.0_f64 * t9637 * t6276 * t20516 - 7.0_f64 / 12.0_f64 * t20522 + t6275 * t2292 * t9505 / 16.0_f64 - t6637 * t20527 * t20528 / 32.0_f64 + t6637 * t6276 * t20532 / 192.0_f64 + 7.0_f64 / 576.0_f64 * t20536;
    (t20527, t20538)
}
