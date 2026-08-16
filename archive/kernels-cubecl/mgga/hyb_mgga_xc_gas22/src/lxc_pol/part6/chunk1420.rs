//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1420/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1420<F: Float>(t1297: F, t3662: F, t9488: F, t2903: F, t3760: F, t13643: F, t3663: F, t1134: F, t9757: F, t1161: F, t2889: F, t4512: F) -> (F, F, F, F, F) {
    let t30723 = t3662 * t9488 * t1297;
    let t30733 = t2903 * t3760;
    let t30736 = t3663 * t13643;
    let t30739 = t1134 * t9757;
    let t30748 = t1161 * t4512 * t2889;
    (t30723, t30733, t30736, t30739, t30748)
}
