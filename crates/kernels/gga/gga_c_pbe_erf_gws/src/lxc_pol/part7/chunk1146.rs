//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1146/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1146<F: Float>(t2305: F, t904: F, t2257: F, t4408: F, t20251: F, t2118: F, t6416: F, t6575: F, t20499: F, t20500: F, t20505: F, t20511: F, t20514: F, t20516: F, t20522: F, t2292: F, t6275: F, t6276: F, t6637: F, t6639: F, t9505: F, t9637: F) -> (F, F) {
    let t20527 = t904 * t2305;
    let t20528 = t4408 * t2257;
    let t20532 = t2118 * t20251;
    let t20536 = t6416 * t6575;
    let t20538 = t20499 + t6637 * t20500 * t6639 / F::cast_from(96.0_f64) + F::cast_from(3.0_f64) / F::cast_from(64.0_f64) * t9637 * t6276 * t20505 + t20511 + t20514 - F::cast_from(3.0_f64) / F::cast_from(128.0_f64) * t9637 * t6276 * t20516 - F::cast_from(7.0_f64) / F::cast_from(12.0_f64) * t20522 + t6275 * t2292 * t9505 / F::cast_from(16.0_f64) - t6637 * t20527 * t20528 / F::cast_from(32.0_f64) + t6637 * t6276 * t20532 / F::cast_from(192.0_f64) + F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t20536;
    (t20527, t20538)
}
