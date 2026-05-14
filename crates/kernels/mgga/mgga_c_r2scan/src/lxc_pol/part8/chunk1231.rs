//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1231/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1231<F: Float>(t26560: F, t4982: F, t959: F, t22320: F, t2743: F, t159: F, t5246: F, t955: F, t1861: F, t7760: F, t2768: F, t5325: F, t1860: F, t5326: F, t7657: F, t22181: F) -> (F, F, F, F, F, F, F, F, F) {
    let t26561 = 0.1714584e0 * t26560;
    let t26562 = t4982 * t959;
    let t26563 = 144.0 * t26562;
    let t26564 = t2743 * t22320;
    let t26567 = t159 * t955 * t5246;
    let t26571 = t7760 * t1861;
    let t26574 = t2768 * t5325;
    let t26575 = t1860 * t26574;
    let t26576 = 0.4051561992e0 * t26575;
    let t26584 = t7657 * t5326;
    let t26585 = 0.4051561992e0 * t26584;
    let t26588 = 36.0 * t22181;
    (t26561, t26563, t26564, t26567, t26571, t26574, t26576, t26585, t26588)
}
