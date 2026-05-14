//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1207/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1207<F: Float>(t5525: F, t5531: F, t17770: F, t1907: F, t140: F, t4594: F, t5598: F, t18717: F, t5439: F, t5444: F, t7528: F, t18672: F, t1993: F, t2441: F, t4971: F, t17960: F, sigma2: F) -> (F, F, F, F, F, F, F, F) {
    let t48513 = t5525 * t5531;
    let t60200 = t17770 * t1907;
    let t60514 = t140 * t5598 * t4594;
    let t60805 = t18717 * t5439;
    let t60823 = t7528 * t5444;
    let t60929 = t18672 * t1993;
    let t61238 = t4971 * t2441;
    let t61353 = t17960 * sigma2;
    (t48513, t60200, t60514, t60805, t60823, t60929, t61238, t61353)
}
