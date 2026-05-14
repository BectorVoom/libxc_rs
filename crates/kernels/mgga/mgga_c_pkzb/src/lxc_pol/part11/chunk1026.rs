//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1026/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1026<F: Float>(t1702: F, t9012: F, t6966: F, t8973: F, t3453: F, t5296: F, t3396: F, t568: F, t16369: F, t8931: F, t5221: F, t8935: F, t8939: F, t16388: F, t3403: F, t3407: F, t5264: F) -> (F, F, F, F, F, F, F, F, F) {
    let t24038 = t1702 * t9012;
    let t24040 = t6966 * t8973;
    let t24054 = t5296 * t3453;
    let t24064 = t3396 * t568;
    let t24075 = t16369 * t8931;
    let t24077 = t5221 * t8935;
    let t24087 = t5221 * t8939;
    let t24089 = t16388 * t3403;
    let t24096 = t5264 * t3407;
    (t24038, t24040, t24054, t24064, t24075, t24077, t24087, t24089, t24096)
}
