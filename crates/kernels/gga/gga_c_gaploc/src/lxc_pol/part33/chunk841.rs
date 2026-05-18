//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 841/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk841<F: Float>(t2779: F, t4614: F, t1323: F, t2787: F, t1445: F, t1603: F, t999: F, t1457: F, t7957: F, t493: F, t7892: F, t590: F) -> (F, F, F, F, F, F) {
    let t8176 = t4614 * t2779;
    let t8179 = t2787 * t1323;
    let t8180 = t1445 * t8179;
    let t8183 = t1603 * t999;
    let t8190 = t1457 * t7957;
    let t8195 = t493 * t7892;
    let t8196 = t8195 * t590;
    (t8176, t8180, t8183, t8190, t8195, t8196)
}
