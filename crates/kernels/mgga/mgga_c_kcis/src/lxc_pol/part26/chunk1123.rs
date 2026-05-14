//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1123/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1123<F: Float>(t18210: F, t28402: F, t7898: F, t27345: F, t8151: F, t27348: F, t28544: F, t1014: F, t28406: F, t7908: F, t98072: F, t28429: F, t28531: F, t1466: F, t5870: F, t491: F, t6019: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t98537 = t18210 * t28402;
    let t98538 = t7898 * t98537;
    let t98566 = t8151 * t27345;
    let t98568 = t8151 * t27348;
    let t98570 = t28544 * t27348;
    let t98573 = t1014 * t28406;
    let t98574 = 0.88437037037037037034e-2 * t98573;
    let t98587 = 0.15445601851851851852e-3 * t7908 * t98072;
    let t98597 = t1014 * t28429;
    let t98598 = 0.33163888888888888888e-2 * t98597;
    let t98603 = t1014 * t28531;
    let t98604 = 0.33163888888888888888e-2 * t98603;
    let t98607 = t5870 * t1466;
    let t98618 = t6019 * t491;
    (t98537, t98538, t98566, t98568, t98570, t98573, t98574, t98587, t98597, t98598, t98603, t98604, t98607, t98618)
}
