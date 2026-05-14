//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1008/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1008<F: Float>(t12556: F, t1561: F, t3275: F, t3277: F, t31510: F, t795: F, t3263: F, t105: F, t3052: F, t97: F, t10669: F, t2526: F, t3574: F, t10610: F, t11479: F, t7040: F) -> (F, F, F, F, F) {
    let t42380 = t1561 * t12556;
    let t42383 = 5.0 / 16.0 * t3275 * t42380 * t3277;
    let t42384 = t31510 * t795;
    let t42387 = 3.0 / 2.0 * t3275 * t3263 * t42384;
    let t42389 = t97 * t105 * t3052;
    let t42391 = 3.0 / 4.0 * t42389 * t10669;
    let t42392 = t3574 * t2526;
    let t42395 = 3.0 * t10610 * t3263 * t42392;
    let t42398 = t3275 * t11479 * t7040 / 2.0;
    (t42383, t42387, t42391, t42395, t42398)
}
