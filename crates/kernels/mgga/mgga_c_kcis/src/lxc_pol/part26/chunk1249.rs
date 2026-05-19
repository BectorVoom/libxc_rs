//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1249/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1249<F: Float>(t1014: F, t28406: F, t7908: F, t98072: F, t28429: F, t28531: F, t1466: F, t5870: F, t491: F, t6019: F, t28388: F, t98137: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t98573 = t1014 * t28406;
    let t98574 = F::cast_from(0.88437037037037037034e-2_f64) * t98573;
    let t98587 = F::cast_from(0.15445601851851851852e-3_f64) * t7908 * t98072;
    let t98597 = t1014 * t28429;
    let t98598 = F::cast_from(0.33163888888888888888e-2_f64) * t98597;
    let t98603 = t1014 * t28531;
    let t98604 = F::cast_from(0.33163888888888888888e-2_f64) * t98603;
    let t98607 = t5870 * t1466;
    let t98618 = t6019 * t491;
    let t98623 = F::cast_from(0.12378114784505208333e-4_f64) * t28388 * t98137;
    (t98573, t98574, t98587, t98597, t98598, t98603, t98604, t98607, t98618, t98623)
}
