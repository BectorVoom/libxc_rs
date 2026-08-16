//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 1012/1331 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk1012<F: Float>(t12574: F, t224: F, t3899: F, t987: F, t3707: F, t435: F, t1736: F, t474: F, t177: F, t208: F, t4913: F, t319: F, t337: F, t7061: F) -> (F, F, F, F, F, F) {
    let t12575 = t224 * t12574;
    let t12664 = t987 * t3899;
    let t12744 = t435 * t3707;
    let t12768 = t474 * t1736;
    let t13281 = t177 / t4913 / t208;
    let t13296 = t319 / t7061 / t337;
    (t12575, t12664, t12744, t12768, t13281, t13296)
}
