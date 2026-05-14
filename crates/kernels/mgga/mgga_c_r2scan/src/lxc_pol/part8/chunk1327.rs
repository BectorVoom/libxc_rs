//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1327/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1327<F: Float>(t2591: F, t32444: F, t3052: F, t481: F, t7338: F, t10049: F, t10099: F, t10316: F, t19820: F, t2122: F, t2133: F, t24208: F, t24674: F, t2531: F, t2598: F, t3090: F, t32348: F, t32353: F, t32373: F, t32377: F, t32424: F, t495: F, t5108: F, t5109: F, t6106: F, t6293: F, t6583: F, t7337: F, t8694: F, t8749: F, t9207: F, t921: F, t9212: F) -> (F, F, F) {
    let t32445 = t32444 * t2591;
    let t32467 = t7338 * t3052 * t481;
    let t32471 = 0.13002332610081402845e0 * t2133 * t5109 * t8694 * t10049 - 0.31205598264195366828e1 * t6106 * t5109 * t32424 - 0.7801399566048841707e0 * t19820 * t10316 - 0.7801399566048841707e0 * t5108 * t5109 * t9207 * t921 - 0.7801399566048841707e0 * t5108 * t5109 * t9212 * t921 - 0.7801399566048841707e0 * t5108 * t5109 * t3090 * t2531 - 0.15602799132097683414e1 * t24674 * t8749 + 0.98781737744032673978e0 * t6293 * t7337 * t32445 + 0.98781737744032673976e0 * t2122 * t24208 * t32373 + 0.78013995660488417068e0 * t2598 * t5109 * t32353 - 0.98781737744032673976e0 * t2122 * t7337 * t32377 + 0.13002332610081402845e0 * t2133 * t5109 * t32348 * t495 - 0.26004665220162805689e0 * t6583 * t5109 * t10099 * t495 - 0.32927245914677557992e0 * t2122 * t7337 * t32467;
    (t32445, t32467, t32471)
}
