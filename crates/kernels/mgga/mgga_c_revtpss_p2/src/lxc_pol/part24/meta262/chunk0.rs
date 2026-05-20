//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1032/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1032<F: Float>(t17395: F, t3717: F, t1284: F, t5219: F, t3624: F, t12879: F, t1715: F, t247: F, t1261: F, t1803: F, t3670: F, t5436: F) -> (F, F, F, F, F, F, F) {
    let t17396 = t3717 * t17395;
    let t17400 = t5219 * t1284;
    let t17401 = t17400 * t3624;
    let t17416 = t247 * t12879 * t1715;
    let t17417 = t1261 * t17416;
    let t17438 = t3670 * t1803;
    let t17448 = t5436 * t3624;
    (t17396, t17400, t17401, t17416, t17417, t17438, t17448)
}
