//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 960/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk960<F: Float>(t2900: F, t7658: F, t302: F, t178: F, t5723: F, t5932: F) -> (F, F, F, F) {
    let t7659 = t2900 * t7658;
    let t7660 = t302 * t7659;
    let t7663 = t5723 * t178;
    let t7664 = t5932 * t7663;
    (t7659, t7660, t7663, t7664)
}
