//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 948/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk948<F: Float>(t7386: F, t7389: F, t5525: F, t5543: F, t5558: F, t5560: F, t5563: F, t5566: F, t7357: F, t7393: F, t7397: F, t7401: F, t7462: F) -> (F, F, F) {
    let t7465 = 0.33114e0 * t7386;
    let t7466 = 0.33114e0 * t7389;
    let t7473 = -0.301925e0 * t5525 + 0.40256666666666666667e0 * t7357 - t7465 - t7466 + 0.248355e0 * t7393 + 0.49671e0 * t7397 + 0.248355e0 * t7401 - t5543 - t5558 + 0.5519e0 * t5560 - 0.16557e0 * t5563 - 0.16557e0 * t5566;
    let t7474 = t7462 + t7473;
    (t7465, t7466, t7474)
}
