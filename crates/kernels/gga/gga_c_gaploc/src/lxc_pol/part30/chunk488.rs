//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 488/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk488<F: Float>(t2482: F, t2488: F, t2487: F, t1339: F, t874: F, t590: F, t1441: F, t1537: F, t1580: F, t1599: F, t193: F, t2428: F, t2434: F, t2437: F, t2441: F, t2446: F, t2449: F, t2452: F, t2457: F, t2460: F, t2468: F, t2472: F, t2474: F, t2480: F, t2484: F, t541: F, t557: F, t574: F, t597: F, t895: F, t904: F, t918: F) -> (F, F, F, F, F) {
    let t2489 = t2488 * t2482;
    let t2490 = t2487 * t2489;
    let t2492 = t1339 * t874;
    let t2493 = t2492 * t590;
    let t2496 = -F::cast_from(0.23005755572352449806e1_f64) * t574 * t2428 + F::cast_from(0.23005755572352449806e1_f64) * t1580 * t918 + F::cast_from(0.23005755572352449806e1_f64) * t597 * t2434 + F::cast_from(0.35750489951850426669e0_f64) * t2437 * t193 + F::cast_from(0.35750489951850426669e0_f64) * t2441 * t193 - F::cast_from(0.35750489951850426669e0_f64) * t1599 * t904 - F::cast_from(0.35750489951850426669e0_f64) * t557 * t2446 - F::cast_from(0.23833659967900284446e0_f64) * t557 * t2449 - F::cast_from(0.30674340763136599741e1_f64) * t574 * t2452 + F::cast_from(0.23833659967900284446e0_f64) * t895 * t541 + F::cast_from(0.51123901271894332902e0_f64) * t1441 * t2457 - F::cast_from(0.95857314884801874192e-1_f64) * t2460 + F::cast_from(0.21301625529955972043e-1_f64) * t2468 - F::cast_from(0.14896037479937677779e-1_f64) * t2472 + F::cast_from(0.14896037479937677779e-1_f64) * t2474 + F::cast_from(0.95857314884801874192e-1_f64) * t2480 - F::cast_from(0.19171462976960374838e0_f64) * t2484 + F::cast_from(0.19171462976960374838e0_f64) * t2490 - F::cast_from(0.51123901271894332902e0_f64) * t1537 * t2493;
    (t2489, t2490, t2492, t2493, t2496)
}
