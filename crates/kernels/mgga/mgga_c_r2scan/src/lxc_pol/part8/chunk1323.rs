//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1323/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1323<F: Float>(t10068: F, t2133: F, t2294: F, t10030: F, t7494: F, t10037: F, t6583: F, t2625: F, t3056: F, t24209: F, t8694: F, t3055: F, t938: F, t7338: F, t28320: F, t921: F) -> (F, F, F, F, F, F, F, F) {
    let t32333 = t2133 * t2294 * t10068;
    let t32335 = t7494 * t10030;
    let t32338 = t6583 * t2294 * t10037;
    let t32340 = t3056 * t2625;
    let t32344 = t24209 * t8694;
    let t32348 = t938 * t3055;
    let t32353 = t7338 * t8694;
    let t32357 = t921 * t28320;
    (t32333, t32335, t32338, t32340, t32344, t32348, t32353, t32357)
}
