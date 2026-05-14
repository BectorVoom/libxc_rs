//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1236/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1236<F: Float>(t2639: F, t568: F, t16369: F, t8931: F, t5221: F, t8935: F, t8939: F, t16388: F, t3403: F, t2575: F, t3407: F, t5264: F, t1719: F, t3401: F, t3441: F, t5391: F) -> (F, F, F, F, F, F, F, F, F) {
    let t24070 = t568 * t2639;
    let t24075 = t16369 * t8931;
    let t24077 = t5221 * t8935;
    let t24087 = t5221 * t8939;
    let t24089 = t16388 * t3403;
    let t24091 = t2575 * t2575;
    let t24096 = t5264 * t3407;
    let t24098 = t3401 * t1719;
    let t24105 = t3441 * t5391 * t1719;
    (t24070, t24075, t24077, t24087, t24089, t24091, t24096, t24098, t24105)
}
