//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1041/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1041<F: Float>(t1143: F, t9573: F, t1539: F, t531: F, t2923: F, t1166: F, t9531: F, t2889: F, t502: F, t7768: F, t1535: F, t2874: F) -> (F, F, F, F, F, F) {
    let t9737 = t1143 * t9573;
    let t9738 = t531 * t1539;
    let t9739 = t9738 * t2923;
    let t9742 = t1166 * t9531;
    let t9747 = t502 * t2889;
    let t9750 = t7768 * t1539;
    let t9757 = t2874 * t1535;
    (t9737, t9739, t9742, t9747, t9750, t9757)
}
