//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 540/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk540<F: Float>(t1725: F, t4903: F, t1704: F, t608: F, t620: F, t4859: F, t4834: F, t4838: F, t4842: F, t4845: F, t4848: F, t1731: F, t45: F, t1736: F, t630: F, t1744: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t4904 = t4903 * t1725;
    let t4907 = t1704 * t1704;
    let t4908 = 1.0 / t4907;
    let t4909 = t608 * t4908;
    let t4910 = t620 * t620;
    let t4911 = 1.0 / t4910;
    let t4912 = t4859 * t4911;
    let t4915 = 0.12361111111111111111e-1 * t4834;
    let t4920 = t4915 + 0.61805555555555555556e-2 * t4838 - 0.61805555555555555555e-2 * t4842 + 0.18541666666666666667e-1 * t4845 - 0.92708333333333333333e-2 * t4848;
    let t4924 = t45 * t1731;
    let t4927 = t1736 * t630;
    let t4928 = 1.0 / t4927;
    let t4929 = t1744 * t1744;
    (t4904, t4907, t4908, t4909, t4910, t4911, t4912, t4915, t4920, t4924, t4928, t4929)
}
