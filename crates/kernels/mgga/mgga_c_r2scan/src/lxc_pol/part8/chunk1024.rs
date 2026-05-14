//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1024/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1024<F: Float>(t10058: F, t8820: F, t360: F, t1569: F, t910: F, t2562: F, t3056: F, t2124: F, t9317: F, t10017: F, t10021: F, t10026: F, t10030: F, t10034: F, t10038: F, t10042: F, t10046: F, t10051: F, t10055: F, t2122: F, t2133: F, t2139: F, t2557: F, t2598: F, t6132: F, t6139: F, t6583: F, t7388: F, t8861: F, t8863: F) -> (F, F, F, F, F, F, F, F) {
    let t10059 = t8820 * t10058;
    let t10060 = t360 * t10059;
    let t10063 = t1569 * t910;
    let t10064 = t8820 * t10063;
    let t10065 = t360 * t10064;
    let t10068 = t2562 * t3056;
    let t10069 = t360 * t10068;
    let t10073 = t2124 * t9317 * t10063;
    let t10079 = 0.39006997830244208535e0 * t2139 * t10017 + 0.13002332610081402845e0 * t2133 * t10021 + 0.26004665220162805689e0 * t2598 * t10026 - 0.82318114786693894983e-1 * t2557 * t10030 + 0.16463622957338778996e0 * t2122 * t10034 - 0.26004665220162805689e0 * t6583 * t10038 + 0.39006997830244208535e0 * t2139 * t10042 + 0.16463622957338778996e0 * t2122 * t10046 + 0.13002332610081402845e0 * t2133 * t10051 + 0.16463622957338778996e0 * t2557 * t10055 - 0.26004665220162805689e0 * t6132 * t10060 - 0.7801399566048841707e0 * t6139 * t10065 + 0.13002332610081402845e0 * t2133 * t10069 - 0.32927245914677557992e0 * t2122 * t10073 - 0.12713391885412927226e1 * t7388 + 0.69345773920434148506e0 * t8861 + 0.38415120233790484326e0 * t8863;
    (t10059, t10060, t10064, t10065, t10068, t10069, t10073, t10079)
}
