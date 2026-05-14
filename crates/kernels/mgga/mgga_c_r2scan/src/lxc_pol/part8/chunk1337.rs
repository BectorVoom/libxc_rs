//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1337/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1337<F: Float>(t10020: F, t2133: F, t2294: F, t7987: F, t9387: F, t10064: F, t6139: F, t10026: F, t2139: F, t2531: F, t2562: F, t25720: F, t29059: F, t29585: F, t3056: F, t32319: F, t360: F, t6121: F, t6583: F, t7512: F, t8001: F, t8022: F, t8773: F, t8775: F, t8825: F, t8832: F, t9110: F, t9207: F, t921: F) -> (F,) {
    let t32718 = t2133 * t2294 * t10020;
    let t32736 = t7987 * t9387;
    let t32739 = t6139 * t2294 * t10064;
    let t32765 = -0.34672886960217074253e0 * t32718 + 0.39006997830244208535e0 * t2139 * t360 * t29059 * t921 - 0.78013995660488417067e0 * t6139 * t360 * t32319 * t6121 + 0.39006997830244208535e0 * t2139 * t360 * t8832 * t2531 - 0.7801399566048841707e0 * t7512 * t360 * t2562 * t9110 - 0.20803732176130244552e1 * t32736 + 0.20803732176130244552e1 * t32739 - 0.15602799132097683414e1 * t7512 * t360 * t2562 * t9207 + 0.13002332610081402845e0 * t2133 * t360 * t29585 * t921 + 0.13002332610081402845e0 * t2133 * t360 * t8825 * t2531 - 0.26004665220162805689e0 * t25720 * t8775 + 0.26004665220162805689e0 * t8022 * t10026 + 0.13002332610081402845e0 * t2133 * t360 * t8001 * t3056 - 0.26004665220162805689e0 * t6583 * t360 * t8773 * t2531;
    (t32765,)
}
