//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 753/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk753<F: Float>(t7335: F, t1861: F, t2759: F, t1873: F, t1073: F, t5511: F, t5547: F, t218: F, t2774: F, t675: F, t2778: F, t1070: F, t1898: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t7359 = 2.0 / 3.0 * t7335;
    let t7365 = t1861 * t2759;
    let t7370 = t1873 * t2759;
    let t7375 = t5511 * t1073;
    let t7378 = t5547 * t1073;
    let t7386 = t218 * t675 * t2774;
    let t7387 = 0.41678e0 * t7386;
    let t7389 = t218 * t675 * t2778;
    let t7390 = 0.41678e0 * t7389;
    let t7411 = t1070 * t1898;
    (t7359, t7365, t7370, t7375, t7378, t7386, t7387, t7389, t7390, t7411)
}
