//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1455/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1455<F: Float>(t322: F, t10533: F, t856: F, t352: F, t9769: F, t35071: F, t10539: F, t1348: F, t10529: F, t10536: F, t10545: F, t11993: F, t1338: F, t19309: F, t19327: F, t2437: F, t2438: F, t2445: F, t2991: F, t31948: F, t31953: F, t3675: F, t8484: F, t8487: F, t855: F, t9760: F, t9763: F, t9773: F, t9778: F) -> (F,) {
    let t332 = 0.25e1 < t322;
    let t35213 = t10533 * t856;
    let t35220 = t352 * t9769;
    let t35231 = piecewise3(t332, t35071, 0.0);
    let t35235 = t1348 * t10539;
    let t35249 = -0.252e2 * t10536 * t2438 - 0.189e2 * t2991 * t9760 - 0.567e2 * t10545 * t2438 - 0.189e2 * t2445 * t35213 - 0.63e1 * t8484 * t10533 - 0.2835e2 * t8487 * t35213 - 0.63e1 * t2437 * t35220 - 0.2835e2 * t9763 * t9760 - 0.2835e2 * t19309 * t10529 * t2438 - 0.21e1 * t1338 * t10539 * t2438 - 0.105e1 * t855 * t35231 * t352 - 0.1575e1 * t35235 * t2438 - 0.4725e1 * t31948 * t3675 - 0.4725e1 * t9773 * t9760 - 0.70875e1 * t31953 * t11993 - 0.70875e1 * t9778 * t9760 - 0.354375e1 * t19327 * t10529 * t2438;
    (t35249,)
}
