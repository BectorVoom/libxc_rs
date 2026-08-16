//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 632/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk632(t1589: f64, t3545: f64, t1339: f64, t3529: f64, t590: f64, t3541: f64, t11343: f64, t11347: f64, t11350: f64, t11353: f64, t11356: f64, t11359: f64, t11362: f64, t11365: f64, t1441: f64, t1537: f64, t1562: f64, t1572: f64, t2386: f64, t4540: f64, t557: f64, t574: f64, t597: f64, t9343: f64) -> (f64, f64) {
    let t11368 = t1589 * t3545;
    let t11371 = t1339 * t3529;
    let t11372 = t11371 * t590;
    let t11375 = t3541 * t590;
    let t11378 = t3545 * t590;
    let t11381 = -0.92023022289409799224e1_f64 * t574 * t11343 + 0.43710935587469654631e2_f64 * t597 * t11347 - 0.62115540045351614476e2_f64 * t1562 * t11350 - 0.21450293971110256001e1_f64 * t4540 * t11353 + 0.71500979903700853338e0_f64 * t1572 * t11356 + 0.42900587942220512003e1_f64 * t11359 * t9343 - 0.10725146985555128001e1_f64 * t11362 * t2386 + 0.47667319935800568892e0_f64 * t1572 * t11365 - 0.23833659967900284446e0_f64 * t557 * t11368 - 0.51123901271894332902e0_f64 * t1537 * t11372 - 0.25561950635947166451e1_f64 * t1537 * t11375 + 0.51123901271894332902e0_f64 * t1441 * t11378;
    (t11371, t11381)
}
