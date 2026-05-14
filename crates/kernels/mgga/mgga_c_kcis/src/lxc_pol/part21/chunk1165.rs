//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1165/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1165<F: Float>(t26739: F, t27915: F, t93163: F, t93686: F, t96318: F, t96321: F, t96324: F, t96327: F, t96330: F, t96333: F, t96340: F, t96342: F, t27870: F, t2822: F, t1087: F, t303: F, t5013: F) -> (F, F, F) {
    let t96344 = 0.27636574074074074073e-2 * t96318 + 0.73697530864197530861e-2 * t96321 + 0.11054629629629629629e-1 * t96324 - 0.33163888888888888888e-2 * t96327 - 0.33163888888888888888e-2 * t96330 - 0.33163888888888888888e-2 * t96333 + 0.29479012345679012345e-2 * t93163 - 0.4946917361111111111e-3 * t26739 * t27915 + 0.15445601851851851852e-3 * t93686 + t96340 - 0.3684876543209876543e-3 * t96342;
    let t96345 = t2822 * t27870;
    let t96354 = t303 * t1087 * t5013;
    (t96344, t96345, t96354)
}
