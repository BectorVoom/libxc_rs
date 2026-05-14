//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1253/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1253<F: Float>(t8739: F, t8923: F, t8618: F, t8906: F, t10638: F, t6574: F, t10641: F, t6497: F, t2234: F, t3356: F, t8853: F, t10658: F, t2228: F, t6562: F, t20829: F, t20832: F, t2189: F, t4113: F) -> (F, F, F, F, F, F, F) {
    let t29402 = 8.0 * t8923 * t8739;
    let t29404 = 0.64327917994770140268e2 * t8906 * t8618;
    let t29406 = 12.0 * t6574 * t10638;
    let t29408 = 8.0 * t6497 * t10641;
    let t29411 = 0.32163958997385070134e2 * t2234 * t3356 * t8853;
    let t29414 = 0.51726012919273400301e3 * t6562 * t10658 * t2228;
    let t29418 = 0.24955700379505800916e5 * t20829 * t4113 * t20832 * t2189;
    (t29402, t29404, t29406, t29408, t29411, t29414, t29418)
}
