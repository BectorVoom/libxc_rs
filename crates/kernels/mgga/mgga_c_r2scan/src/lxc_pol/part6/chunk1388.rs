//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1388/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1388<F: Float>(t20762: F, t25562: F, t538: F, t2573: F, t481: F, t20594: F, t25767: F, t10855: F, t25850: F, t489: F, t25300: F, t20791: F, t6161: F, t923: F, t2173: F, t2562: F, t26295: F, t26297: F, t26298: F, t26301: F, t26302: F, t26306: F, t26309: F, t2670: F, t360: F, t5155: F, t6334: F, t7250: F, t7461: F) -> (F,) {
    let t26312 = t20762 * t538 * t25562;
    let t26314 = t2573 * t481;
    let t26316 = t20594 * t25767 * t26314;
    let t26319 = t25850 * t10855 * t489;
    let t26320 = t26319 * t25300;
    let t26327 = t20791 * t923 * t6161;
    let t26333 = t26295 + t26297 - 0.35126785941778018868e0 * t26298 + t26301 + 0.17563392970889009434e0 * t26302 + t26306 - 0.17465477326173296717e-1 * t26309 + 0.49390868872016336989e-1 * t26312 - 0.1047928639570397803e0 * t26316 - 0.17563392970889009434e0 * t26320 - 0.39006997830244208535e0 * t7250 * t2173 - 0.13002332610081402845e0 * t2670 * t5155 - 0.51410067763503603055e-4 * t26327 - 0.15602799132097683414e1 * t7461 * t360 * t2562 * t6334;
    (t26333,)
}
