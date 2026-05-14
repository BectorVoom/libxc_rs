//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1097/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1097<F: Float>(t32356: F, t7290: F, t1841: F, t7289: F, t2554: F, t7064: F, t9006: F, t10667: F, t296: F, t10714: F, t7137: F, t10782: F, t1710: F, t10673: F, t10722: F, t123: F, t1836: F, t1897: F, t2060: F, t2508: F, t2580: F, t29349: F, t29354: F, t32351: F, t32353: F, t32355: F, t3433: F, t3464: F, t5288: F, t734: F, t779: F) -> (F, F, F, F) {
    let t32357 = t7290 * t32356;
    let t32360 = 0.34180116578409885704e-2 * t1841 * t7289 * t32357;
    let t32362 = t7064 * t9006 * t2554;
    let t32363 = 0.64087718584518535698e-3 * t32362;
    let t32364 = t296 * t10667;
    let t32370 = 0.41016139894091862846e-1 * t7137 * t10714;
    let t32371 = t10782 * t1710;
    let t32386 = t29349 + t32351 - t32353 - t32355 - t32360 + t29354 - t32363 - 0.17090058289204942853e-2 * t1841 * t32364 * t123 * t734 + t32370 + 0.15381052460284448567e-1 * t2508 * t2580 * t32371 + 0.15381052460284448567e-1 * t5288 * t10722 + 0.15381052460284448567e-1 * t2508 * t779 * t10673 + 0.76905262301422242837e-2 * t2508 * t2060 * t3433 - 0.76905262301422242837e-2 * t1897 * t3464 * t1836;
    (t32357, t32364, t32371, t32386)
}
