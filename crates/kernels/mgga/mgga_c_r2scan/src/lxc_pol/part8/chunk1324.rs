//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1324/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1324<F: Float>(t10024: F, t7604: F, t28404: F, t3071: F, t20997: F, t21003: F, t2122: F, t24208: F, t2557: F, t2573: F, t27678: F, t32333: F, t32335: F, t32338: F, t32340: F, t32344: F, t32348: F, t32353: F, t32357: F, t5109: F, t6132: F, t6139: F, t6293: F, t6583: F, t7321: F, t7337: F, t7921: F) -> (F, F, F, F) {
    let t32365 = t10024 * t7604;
    let t32373 = t28404 * t7604;
    let t32377 = t3071 * t7604;
    let t32381 = 0.34672886960217074253e0 * t27678 - 0.34672886960217074253e0 * t32333 + 0.19207560116895242163e0 * t32335 + 0.69345773920434148504e0 * t32338 - 0.49390868872016336988e0 * t6293 * t7321 * t32340 - 0.49390868872016336988e0 * t2557 * t24208 * t32344 - 0.2600466522016280569e0 * t6583 * t5109 * t32348 * t2573 + 0.49390868872016336989e0 * t2557 * t7337 * t32353 + 0.32927245914677557992e0 * t2122 * t7321 * t32357 + 0.78013995660488417067e0 * t20997 * t5109 * t28404 * t7921 - 0.32927245914677557992e0 * t2122 * t7337 * t32365 - 0.78013995660488417068e0 * t6132 * t5109 * t3071 * t7921 + 0.23404198698146525121e1 * t21003 * t5109 * t32373 - 0.23404198698146525121e1 * t6139 * t5109 * t32377;
    (t32365, t32373, t32377, t32381)
}
