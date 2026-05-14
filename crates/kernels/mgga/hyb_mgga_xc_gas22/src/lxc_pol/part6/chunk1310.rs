//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1310/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1310<F: Float>(t1539: F, t5471: F, t1304: F, t3663: F, t1129: F, t4489: F, t2875: F, t30603: F, t1123: F, t1144: F, t1149: F, t26727: F, t26883: F, t26886: F, t2927: F, t30574: F, t30578: F, t30586: F, t30600: F, t9636: F, t9646: F, t9650: F, t9657: F, t9660: F) -> (F, F, F, F) {
    let t30611 = t5471 * t1539;
    let t30615 = t3663 * t1304;
    let t30616 = t4489 * t1129;
    let t30617 = t30615 * t30616;
    let t30620 = t2875 * t30603;
    let t30641 = t4489 * t1123;
    let t30642 = t30615 * t30641;
    let t30648 = -1792.0 / 27.0 * t2875 * t30611 * t9657 - 11200.0 / 27.0 * t26883 * t30617 - 896.0 / 9.0 * t30620 * t9660 - 11200.0 / 27.0 * t30620 * t9636 + 12800.0 / 243.0 * t1149 * t30574 * t30578 + 12800.0 / 243.0 * t1144 * t30574 * t30578 + 6400.0 / 81.0 * t1144 * t26727 * t30586 - 3200.0 / 81.0 * t26886 * t30617 + 256.0 / 27.0 * t30600 * t9660 + 512.0 / 81.0 * t2927 * t30611 * t9646 + 3200.0 / 81.0 * t26886 * t30642 + 256.0 / 27.0 * t2927 * t30603 * t9650;
    (t30611, t30617, t30642, t30648)
}
