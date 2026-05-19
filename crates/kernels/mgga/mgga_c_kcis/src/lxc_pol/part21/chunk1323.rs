//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1323/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1323<F: Float>(t1020: F, t13151: F, t7718: F, t27867: F, t2822: F, t13435: F, t26739: F, t27915: F, t93163: F, t93686: F, t96318: F, t96321: F, t96324: F, t96327: F, t96330: F) -> (F, F, F, F) {
    let t96333 = t1020 * t7718 * t13151;
    let t96339 = t2822 * t27867;
    let t96340 = F::cast_from(0.14739506172839506172e-2_f64) * t96339;
    let t96342 = t1020 * t7718 * t13435;
    let t96344 = F::cast_from(0.27636574074074074073e-2_f64) * t96318 + F::cast_from(0.73697530864197530861e-2_f64) * t96321 + F::cast_from(0.11054629629629629629e-1_f64) * t96324 - F::cast_from(0.33163888888888888888e-2_f64) * t96327 - F::cast_from(0.33163888888888888888e-2_f64) * t96330 - F::cast_from(0.33163888888888888888e-2_f64) * t96333 + F::cast_from(0.29479012345679012345e-2_f64) * t93163 - F::cast_from(0.4946917361111111111e-3_f64) * t26739 * t27915 + F::cast_from(0.15445601851851851852e-3_f64) * t93686 + t96340 - F::cast_from(0.3684876543209876543e-3_f64) * t96342;
    (t96333, t96339, t96342, t96344)
}
