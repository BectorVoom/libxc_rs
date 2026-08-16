//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1323/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1323(t1020: f64, t13151: f64, t7718: f64, t27867: f64, t2822: f64, t13435: f64, t26739: f64, t27915: f64, t93163: f64, t93686: f64, t96318: f64, t96321: f64, t96324: f64, t96327: f64, t96330: f64) -> (f64, f64, f64, f64) {
    let t96333 = t1020 * t7718 * t13151;
    let t96339 = t2822 * t27867;
    let t96340 = 0.14739506172839506172e-2_f64 * t96339;
    let t96342 = t1020 * t7718 * t13435;
    let t96344 = 0.27636574074074074073e-2_f64 * t96318 + 0.73697530864197530861e-2_f64 * t96321 + 0.11054629629629629629e-1_f64 * t96324 - 0.33163888888888888888e-2_f64 * t96327 - 0.33163888888888888888e-2_f64 * t96330 - 0.33163888888888888888e-2_f64 * t96333 + 0.29479012345679012345e-2_f64 * t93163 - 0.4946917361111111111e-3_f64 * t26739 * t27915 + 0.15445601851851851852e-3_f64 * t93686 + t96340 - 0.3684876543209876543e-3_f64 * t96342;
    (t96333, t96339, t96342, t96344)
}
