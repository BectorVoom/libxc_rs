//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1096/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1096<F: Float>(t20788: F, t672: F, t665: F, t1862: F, t2759: F, t5511: F, t5547: F, t1073: F, t17432: F, t5512: F, t17444: F, t1873: F, t667: F, t7360: F, t1861: F, t1867: F, t7365: F) -> (F, F, F, F, F, F, F, F, F) {
    let t20789 = t672 * t20788;
    let t20791 = t665 * t20788;
    let t20794 = t5511 * t2759 * t1862;
    let t20797 = t5547 * t2759 * t1862;
    let t20800 = t17432 * t1073 * t5512;
    let t20803 = t17444 * t1073 * t5512;
    let t20806 = t1873 * t7360 * t667;
    let t20809 = t1861 * t7360 * t667;
    let t20811 = t7365 * t1867;
    (t20789, t20791, t20794, t20797, t20800, t20803, t20806, t20809, t20811)
}
