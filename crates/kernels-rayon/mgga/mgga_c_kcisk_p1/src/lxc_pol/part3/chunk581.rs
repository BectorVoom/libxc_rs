//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 581/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk581(t1725: f64, t4903: f64, t1704: f64, t608: f64, t620: f64, t4859: f64, t4834: f64, t4838: f64, t4842: f64, t4845: f64, t4848: f64, t1731: f64, t45: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4904 = t4903 * t1725;
    let t4907 = t1704 * t1704;
    let t4908 = 1.0_f64 / t4907;
    let t4909 = t608 * t4908;
    let t4910 = t620 * t620;
    let t4911 = 1.0_f64 / t4910;
    let t4912 = t4859 * t4911;
    let t4915 = 0.12361111111111111111e-1_f64 * t4834;
    let t4920 = t4915 + 0.61805555555555555556e-2_f64 * t4838 - 0.61805555555555555555e-2_f64 * t4842 + 0.18541666666666666667e-1_f64 * t4845 - 0.92708333333333333333e-2_f64 * t4848;
    let t4924 = t45 * t1731;
    (t4904, t4907, t4908, t4909, t4910, t4911, t4912, t4920, t4924)
}
