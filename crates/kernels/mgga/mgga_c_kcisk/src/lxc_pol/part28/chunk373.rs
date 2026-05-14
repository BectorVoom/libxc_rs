//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 373/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk373<F: Float>(t1729: F, t2402: F, t1739: F, t1742: F, t2409: F, t2412: F, t2415: F, t1737: F, t1746: F, t1706: F, t1735: F, t2404: F, t2418: F, t45: F, t621: F, t634: F) -> (F, F, F, F) {
    let t2422 = -t1729 - 0.92708333333333333333e-2 * t2402;
    let t2430 = 0.258925e1 * t2409 - t1739 - 0.301925e0 * t2402 + 0.16504875e0 * t2412 - t1742 - 0.16557e0 * t2415;
    let t2432 = t1737 * t2430 * t1746;
    let t2435 = -0.62182e-1 * t2404 * t621 + 1.0 * t1706 * t2418 + 0.19751789702565206229e-1 * t45 * t2422 * t634 - 0.58482233974552040708e0 * t1735 * t2432;
    (t2422, t2430, t2432, t2435)
}
