//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 988/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk988<F: Float>(t4873: F, t7115: F, t4864: F, t7107: F, t1709: F, t7099: F, t4881: F, t17402: F, t17399: F, t1224: F, t16004: F, t4840: F, t16017: F, t1697: F, t10937: F, t10941: F, t10944: F, t10947: F, t11040: F, t17379: F, t17382: F, t17405: F, t17408: F, t17412: F, t17417: F, t17420: F, t17435: F) -> (F, F, F, F, F, F, F) {
    let t17437 = t7115 * t4873;
    let t17439 = t4864 * t7107;
    let t17440 = t17439 * t1709;
    let t17442 = t7099 * t4873;
    let t17445 = t4881 * t7107;
    let t17446 = t17445 * t1709;
    let t17453 = 4.0 / 27.0 * t17402;
    let t17454 = 4.0 / 9.0 * t17399;
    let t17458 = t1224 * t4840 * t16004;
    let t17463 = t1224 * t1697 * t16017;
    let t17468 = -t11040 - 8.0 / 27.0 * t10937 + 2.0 / 27.0 * t10941 - 2.0 / 9.0 * t10944 + t10947 / 9.0 - 4.0 / 27.0 * t17382 + t17453 - t17454 - 22.0 / 9.0 * t17379 - 10.0 / 27.0 * t17408 + 4.0 / 3.0 * t17458 + 8.0 / 9.0 * t17412 - 2.0 / 9.0 * t17405 - 2.0 * t17463 - 8.0 / 3.0 * t17420 + 2.0 / 3.0 * t17417 + 2.0 / 3.0 * t17435;
    (t17437, t17440, t17442, t17446, t17458, t17463, t17468)
}
