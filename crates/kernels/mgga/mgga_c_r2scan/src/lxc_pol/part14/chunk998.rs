//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 998/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk998<F: Float>(t10777: F, t39375: F, t2650: F, t565: F, t10773: F, t11802: F, t37685: F, t3308: F, t574: F, t7940: F, t11797: F, t1584: F, t10776: F, t7442: F, t10772: F, t7449: F) -> (F, F, F, F, F, F, F) {
    let t39376 = t39375 * t10777;
    let t39378 = t565 * t2650;
    let t39379 = t39378 * t10773;
    let t39381 = t37685 * t11802;
    let t39385 = t574 * t3308 * t7940;
    let t39387 = t1584 * t11797;
    let t39390 = t10776 * t3308 * t7442;
    let t39393 = t10772 * t3308 * t7449;
    (t39376, t39379, t39381, t39385, t39387, t39390, t39393)
}
