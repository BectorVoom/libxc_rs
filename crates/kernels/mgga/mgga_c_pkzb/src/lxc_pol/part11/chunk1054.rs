//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1054/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1054<F: Float>(t154: F, t3730: F, t385: F, t6446: F, t28063: F, t6524: F, t6456: F, t10212: F, t10214: F, t2380: F, t54: F, t10208: F, t23213: F, t3185: F, t10204: F, t3206: F) -> (F, F, F, F, F, F) {
    let t28174 = t385 * t154 * t6446 * t3730;
    let t28188 = t6524 * t28063;
    let t28195 = t6456 * t28063;
    let t28227 = t2380 * t54 * t10212 * t10214;
    let t28231 = t3185 * t23213 * t10208;
    let t28234 = t3206 * t23213 * t10204;
    (t28174, t28188, t28195, t28227, t28231, t28234)
}
