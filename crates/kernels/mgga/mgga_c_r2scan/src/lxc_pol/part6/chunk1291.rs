//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1291/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1291<F: Float>(t2551: F, t7197: F, t2139: F, t22709: F, t7356: F, t6106: F, t7322: F, t5068: F, t7338: F, t19878: F, t495: F, t7591: F, t2533: F, t7378: F, t20127: F, t20313: F, t20997: F, t2122: F, t2133: F, t22783: F, t24209: F, t24228: F, t2557: F, t5109: F, t6219: F, t6293: F, t7321: F, t7337: F, t7349: F, t7428: F, t8170: F, t921: F) -> (F, F, F, F, F) {
    let t24233 = t7197 * t2551;
    let t24238 = t2139 * t22709 * t7356;
    let t24241 = t6106 * t22709 * t7322;
    let t24245 = t7338 * t5068;
    let t24253 = t7338 * t19878;
    let t24268 = t7591 * t495;
    let t24276 = t2533 * t7378;
    let t24283 = 0.7801399566048841707e0 * t2139 * t5109 * t24233 - 0.20803732176130244552e1 * t24238 + 0.83214928704520978207e1 * t24241 - 0.15602799132097683414e1 * t22783 * t7349 + 0.49390868872016336988e0 * t2557 * t7337 * t24245 + 0.7801399566048841707e0 * t20127 * t5109 * t8170 * t495 + 0.98781737744032673978e0 * t6293 * t7337 * t24253 + 0.78013995660488417068e0 * t20997 * t5109 * t24209 * t20313 - 0.49390868872016336991e0 * t6293 * t7321 * t24228 + 0.13002332610081402845e0 * t2133 * t5109 * t2533 * t7428 - 0.31205598264195366828e1 * t6106 * t5109 * t24268 + 0.7801399566048841707e0 * t20127 * t5109 * t921 * t6219 - 0.49390868872016336989e0 * t6293 * t7321 * t24276 + 0.32927245914677557992e0 * t2122 * t7321 * t24233;
    (t24245, t24253, t24268, t24276, t24283)
}
