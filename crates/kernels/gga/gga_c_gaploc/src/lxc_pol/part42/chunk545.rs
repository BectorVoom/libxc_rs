//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 545/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk545<F: Float>(t3541: F, t590: F, t3545: F, t11343: F, t11347: F, t11350: F, t11353: F, t11356: F, t11359: F, t11362: F, t11365: F, t11368: F, t11372: F, t1441: F, t1537: F, t1562: F, t1572: F, t2386: F, t4540: F, t557: F, t574: F, t597: F, t9343: F) -> (F,) {
    let t11375 = t3541 * t590;
    let t11378 = t3545 * t590;
    let t11381 = -0.92023022289409799224e1 * t574 * t11343 + 0.43710935587469654631e2 * t597 * t11347 - 0.62115540045351614476e2 * t1562 * t11350 - 0.21450293971110256001e1 * t4540 * t11353 + 0.71500979903700853338e0 * t1572 * t11356 + 0.42900587942220512003e1 * t11359 * t9343 - 0.10725146985555128001e1 * t11362 * t2386 + 0.47667319935800568892e0 * t1572 * t11365 - 0.23833659967900284446e0 * t557 * t11368 - 0.51123901271894332902e0 * t1537 * t11372 - 0.25561950635947166451e1 * t1537 * t11375 + 0.51123901271894332902e0 * t1441 * t11378;
    (t11381,)
}
