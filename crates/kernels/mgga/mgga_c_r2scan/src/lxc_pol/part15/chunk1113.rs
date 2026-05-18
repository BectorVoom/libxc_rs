//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1113/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1113<F: Float>(t10772: F, t3308: F, t8106: F, t37782: F, t8111: F, t574: F, t7453: F, t2650: F, t546: F, t10777: F, t565: F, t10773: F) -> (F, F, F, F, F) {
    let t39367 = t10772 * t3308 * t8106;
    let t39370 = t37782 * t3308 * t8111;
    let t39373 = t574 * t3308 * t7453;
    let t39375 = t546 * t2650;
    let t39376 = t39375 * t10777;
    let t39378 = t565 * t2650;
    let t39379 = t39378 * t10773;
    (t39367, t39370, t39373, t39376, t39379)
}
