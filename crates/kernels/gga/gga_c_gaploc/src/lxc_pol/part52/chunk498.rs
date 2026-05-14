//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 498/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk498<F: Float>(t10455: F, t1572: F, t3384: F, t4950: F, t10140: F, t1457: F, t3395: F, t6985: F, t2487: F, t10241: F, t1339: F, t590: F, t1537: F, t493: F, t1441: F, t10144: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t10457 = 0.47667319935800568892e0 * t1572 * t10455;
    let t10459 = 0.71500979903700853338e0 * t4950 * t3384;
    let t10463 = t1457 * t10140;
    let t10465 = 0.71500979903700853338e0 * t1572 * t10463;
    let t10466 = t6985 * t3395;
    let t10467 = t2487 * t10466;
    let t10468 = 0.25561950635947166451e0 * t10467;
    let t10469 = t1339 * t10241;
    let t10470 = t10469 * t590;
    let t10472 = 0.25561950635947166451e1 * t1537 * t10470;
    let t10473 = t493 * t10241;
    let t10474 = t10473 * t590;
    let t10476 = 0.1022478025437886658e1 * t1441 * t10474;
    let t10477 = t1457 * t10144;
    (t10457, t10459, t10463, t10465, t10467, t10468, t10469, t10472, t10473, t10476, t10477)
}
