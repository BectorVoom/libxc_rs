//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1350/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1350<F: Float>(t2242: F, t9837: F, t2240: F, t851: F, t10176: F, t2328: F, t10150: F, t2298: F, t898: F, t3147: F, t8013: F, t22391: F, t8199: F, t22180: F, t26848: F, t8177: F) -> (F, F, F, F, F, F) {
    let t26880 = t9837 * t2242;
    let t26883 = 0.32163958997385070134e2 * t2240 * t26880 * t851;
    let t26885 = 0.23392894490538584828e1 * t2328 * t10176;
    let t26888 = 0.14035736694323150897e2 * t898 * t10150 * t2298;
    let t26890 = 0.69263436422725855034e2 * t3147 * t8013;
    let t26892 = 0.38596750796862084161e3 * t22391 * t8199;
    let t26895 = 0.41016075432865626631e4 * t22180 * t8177 * t26848;
    (t26883, t26885, t26888, t26890, t26892, t26895)
}
