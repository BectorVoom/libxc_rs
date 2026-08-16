//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 312/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk312(t2492: f64, t590: f64, t1441: f64, t1537: f64, t1580: f64, t1599: f64, t193: f64, t2428: f64, t2434: f64, t2437: f64, t2441: f64, t2446: f64, t2449: f64, t2452: f64, t2457: f64, t2460: f64, t2468: f64, t2472: f64, t2474: f64, t2480: f64, t2484: f64, t2490: f64, t541: f64, t557: f64, t574: f64, t597: f64, t895: f64, t904: f64, t918: f64) -> f64 {
    let t2493 = t2492 * t590;
    let t2496 = -0.23005755572352449806e1_f64 * t574 * t2428 + 0.23005755572352449806e1_f64 * t1580 * t918 + 0.23005755572352449806e1_f64 * t597 * t2434 + 0.35750489951850426669e0_f64 * t2437 * t193 + 0.35750489951850426669e0_f64 * t2441 * t193 - 0.35750489951850426669e0_f64 * t1599 * t904 - 0.35750489951850426669e0_f64 * t557 * t2446 - 0.23833659967900284446e0_f64 * t557 * t2449 - 0.30674340763136599741e1_f64 * t574 * t2452 + 0.23833659967900284446e0_f64 * t895 * t541 + 0.51123901271894332902e0_f64 * t1441 * t2457 - 0.95857314884801874192e-1_f64 * t2460 + 0.21301625529955972043e-1_f64 * t2468 - 0.14896037479937677779e-1_f64 * t2472 + 0.14896037479937677779e-1_f64 * t2474 + 0.95857314884801874192e-1_f64 * t2480 - 0.19171462976960374838e0_f64 * t2484 + 0.19171462976960374838e0_f64 * t2490 - 0.51123901271894332902e0_f64 * t1537 * t2493;
    t2496
}
