//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 771/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk771<F: Float>(t5218: F, t9967: F, t2559: F, t736: F, t2527: F, t7316: F, t9704: F, t2568: F, t733: F, t2441: F, t5290: F, t9708: F, t6719: F, t748: F, t1873: F, t2580: F) -> (F, F, F, F, F, F, F, F, F) {
    let t9969 = 2.0 * t5218 * t9967;
    let t9970 = t2559 * t736;
    let t9972 = t7316 * t2527;
    let t9973 = t9704 * t9972;
    let t9975 = t733 * t2568;
    let t9977 = t5290 * t2441;
    let t9978 = t9708 * t9977;
    let t9980 = t6719 * t748;
    let t9982 = t1873 * t2580;
    (t9969, t9970, t9972, t9973, t9975, t9977, t9978, t9980, t9982)
}
