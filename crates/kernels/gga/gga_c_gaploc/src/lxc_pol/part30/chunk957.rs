//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 957/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk957<F: Float>(t198: F, t20157: F, t565: F, t595: F, t1560: F, t4360: F, t4390: F, t4250: F, t874: F, t20073: F, t2366: F, t10523: F, t1422: F, t544: F, t1564: F, t165: F) -> (F, F, F, F, F, F, F, F) {
    let t20158 = t565 * t198 * t20157;
    let t20168 = t565 * t595 * t20157;
    let t20172 = t565 * t1560 * t20157;
    let t20229 = t4360 * t4390;
    let t20237 = t4250 * t874;
    let t20358 = t2366 * t20073;
    let t20367 = t544 * t10523 * t1422;
    let t20368 = t165 * t1564;
    (t20158, t20168, t20172, t20229, t20237, t20358, t20367, t20368)
}
