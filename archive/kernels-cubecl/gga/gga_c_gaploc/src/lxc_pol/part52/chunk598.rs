//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 598/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk598<F: Float>(t1589: F, t3545: F, t1339: F, t3529: F, t590: F, t3541: F, t11343: F, t11347: F, t11350: F, t11353: F, t11356: F, t11359: F, t11362: F, t11365: F, t1441: F, t1537: F, t1562: F, t1572: F, t2386: F, t4540: F, t557: F, t574: F, t597: F, t9343: F) -> (F, F) {
    let t11368 = t1589 * t3545;
    let t11371 = t1339 * t3529;
    let t11372 = t11371 * t590;
    let t11375 = t3541 * t590;
    let t11378 = t3545 * t590;
    let t11381 = -F::cast_from(0.92023022289409799224e1_f64) * t574 * t11343 + F::cast_from(0.43710935587469654631e2_f64) * t597 * t11347 - F::cast_from(0.62115540045351614476e2_f64) * t1562 * t11350 - F::cast_from(0.21450293971110256001e1_f64) * t4540 * t11353 + F::cast_from(0.71500979903700853338e0_f64) * t1572 * t11356 + F::cast_from(0.42900587942220512003e1_f64) * t11359 * t9343 - F::cast_from(0.10725146985555128001e1_f64) * t11362 * t2386 + F::cast_from(0.47667319935800568892e0_f64) * t1572 * t11365 - F::cast_from(0.23833659967900284446e0_f64) * t557 * t11368 - F::cast_from(0.51123901271894332902e0_f64) * t1537 * t11372 - F::cast_from(0.25561950635947166451e1_f64) * t1537 * t11375 + F::cast_from(0.51123901271894332902e0_f64) * t1441 * t11378;
    (t11371, t11381)
}
