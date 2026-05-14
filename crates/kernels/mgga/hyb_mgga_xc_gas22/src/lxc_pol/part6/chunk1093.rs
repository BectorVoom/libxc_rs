//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1093/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1093<F: Float>(t17: F, t7692: F, t7768: F, t2850: F, t412: F, t11406: F, t3957: F, t126: F, t19: F, t8184: F, t547: F, t5888: F, t2986: F, t641: F, t669: F, t1815: F, t1862: F) -> (F, F, F, F, F, F, F, F, F) {
    let t15041 = t7692 * t17;
    let t15052 = t7768 * t17;
    let t15681 = t2850 * t412;
    let t15686 = t11406 * t3957;
    let t19557 = 5.0 / 108.0 * t19 * t8184 * t126;
    let t19568 = t547 * t5888;
    let t19571 = t19 * t2986 * t641;
    let t19574 = t19 * t2986 * t669;
    let t19577 = t19 * t1815 * t1862;
    (t15041, t15052, t15681, t15686, t19557, t19568, t19571, t19574, t19577)
}
