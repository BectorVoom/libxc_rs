//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 941/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk941<F: Float>(t7335: F, t5520: F, t5522: F, t5525: F, t7352: F, t7357: F, t672: F, t665: F, t1861: F, t2759: F, t667: F, t1867: F, t2754: F, t1873: F, t2765: F, t1073: F, t5511: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7359 = 2.0 / 3.0 * t7335;
    let t7360 = -t5520 + 8.0 / 9.0 * t5522 - t5525 / 3.0 + 4.0 / 9.0 * t7357 - t7359 + t7352;
    let t7361 = t672 * t7360;
    let t7363 = t665 * t7360;
    let t7365 = t1861 * t2759;
    let t7366 = t7365 * t667;
    let t7368 = t2754 * t1867;
    let t7370 = t1873 * t2759;
    let t7371 = t7370 * t667;
    let t7373 = t2765 * t1867;
    let t7375 = t5511 * t1073;
    (t7359, t7360, t7361, t7363, t7366, t7368, t7371, t7373, t7375)
}
