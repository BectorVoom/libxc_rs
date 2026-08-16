//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1124/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1124<F: Float>(t5221: F, t8939: F, t16388: F, t3403: F, t3407: F, t5264: F, t17043: F, t8978: F, t6892: F, t8921: F, t5257: F, t8964: F) -> (F, F, F, F, F, F) {
    let t24087 = t5221 * t8939;
    let t24089 = t16388 * t3403;
    let t24096 = t5264 * t3407;
    let t24135 = t17043 * t8978;
    let t24137 = t6892 * t8921;
    let t24155 = t5257 * t8964;
    (t24087, t24089, t24096, t24135, t24137, t24155)
}
