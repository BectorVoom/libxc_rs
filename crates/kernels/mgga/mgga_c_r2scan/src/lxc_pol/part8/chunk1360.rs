//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1360/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1360<F: Float>(t113: F, t32348: F, t6085: F, t6086: F, t2133: F, t2139: F, t2551: F, t2573: F, t29381: F, t29392: F, t29394: F, t29405: F, t29409: F, t29411: F, t29415: F, t32319: F, t360: F, t7984: F, t7987: F, t9140: F, t9144: F, t9262: F) -> (F,) {
    let t33281 = t32348 * t113;
    let t33283 = t6085 * t6086 * t33281;
    let t33285 = 0.43341108700271342816e-1 * t2133 * t360 * t32319 * t2573 + 0.13002332610081402845e0 * t2139 * t360 * t32319 * t2551 + 0.7801399566048841707e0 * t7987 * t9262 + 0.13002332610081402845e0 * t7984 * t9140 + 0.39006997830244208535e0 * t7987 * t9144 - 0.38415120233790484326e0 * t29381 - 0.69345773920434148506e0 * t29392 - 0.69345773920434148506e0 * t29394 + 0.11524536070137145298e1 * t29405 + 0.69345773920434148506e0 * t29409 - 0.76830240467580968651e0 * t29411 - 0.40752780427737692339e0 * t29415 + 0.17465477326173296717e-1 * t33283;
    (t33285,)
}
