//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 713/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk713<F: Float>(t4811: F, t4862: F, t1481: F, t382: F, t14: F, t1484: F, t31: F, t4824: F) -> (F, F, F, F, F, F, F) {
    let t4863 = t4811 * t4862;
    let t4867 = 1.0 / t1481 / t382;
    let t4868 = t14 * t4867;
    let t4870 = 1.0 / t1484 / t31;
    let t4871 = t4824 * t4870;
    let t4872 = t4868 * t4871;
    let t4873 = 0.51726012919273400301e3 * t4872;
    (t4863, t4867, t4868, t4870, t4871, t4872, t4873)
}
