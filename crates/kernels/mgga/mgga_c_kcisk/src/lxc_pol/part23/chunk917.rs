//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 917/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk917<F: Float>(t18968: F, t3482: F, t2262: F, t3575: F, t3796: F, t5633: F, t3508: F, t6002: F, t1411: F, t13311: F, t5621: F, t5627: F, t3483: F, t3776: F, t1163: F, t6006: F) -> (F, F, F, F, F, F, F, F) {
    let t18969 = t3482 * t18968;
    let t18971 = t2262 * t3575;
    let t18972 = t3796 * t18971;
    let t18973 = t5633 * t18972;
    let t18975 = t3508 * t6002;
    let t18976 = t1411 * t18975;
    let t18978 = t13311 * t5621;
    let t18979 = t3482 * t18978;
    let t18981 = t13311 * t5627;
    let t18982 = t3482 * t18981;
    let t18984 = t3483 * t3776;
    let t18985 = t6006 * t1163;
    (t18969, t18971, t18973, t18976, t18979, t18982, t18984, t18985)
}
