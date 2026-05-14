//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1273/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1273<F: Float>(t7360: F, t7987: F, t2294: F, t2598: F, t9529: F, t3216: F, t494: F, t113: F, t6085: F, t6086: F, t2106: F, t3179: F, t3197: F, t625: F, t6069: F, t20450: F, t20473: F, t3186: F) -> (F, F, F, F, F, F, F) {
    let t29680 = t7987 * t7360;
    let t29692 = t2598 * t2294 * t9529;
    let t29699 = t3216 * t494;
    let t29700 = t29699 * t113;
    let t29702 = t6085 * t6086 * t29700;
    let t29704 = t3179 * t2106;
    let t29706 = t3197 * t625;
    let t29707 = t6069 * t29706;
    let t29710 = t20450 * t20473 * t3186;
    (t29680, t29692, t29702, t29704, t29706, t29707, t29710)
}
