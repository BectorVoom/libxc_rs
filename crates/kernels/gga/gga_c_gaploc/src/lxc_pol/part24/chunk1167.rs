//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1167/1270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1167<F: Float>(t3484: F, t6021: F, t10973: F, t2194: F, t32435: F, t701: F, t6066: F, t7630: F, t32356: F, t739: F, t1991: F, t590: F, t10938: F, t2021: F, t23310: F, t25177: F, t959: F) -> (F, F, F, F, F, F, F) {
    let t33544 = 0.46011511144704899612e1 * t6021 * t3484;
    let t33546 = 0.92023022289409799224e1 * t2194 * t10973;
    let t33557 = t32435 * t701;
    let t33560 = 0.14300195980740170668e1 * t7630 * t6066 * t33557;
    let t33561 = t739 * t32356;
    let t33564 = 0.2044956050875773316e1 * t1991 * t33561 * t590;
    let t33565 = t2021 * t10938;
    let t33567 = 0.79445533226334281486e-1 * t33565 * t23310;
    let t33568 = t25177 * t959;
    (t33544, t33546, t33557, t33560, t33564, t33567, t33568)
}
