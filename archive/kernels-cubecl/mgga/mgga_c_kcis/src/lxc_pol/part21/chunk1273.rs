//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1273/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1273<F: Float>(t10491: F, t28005: F, t14668: F, t26883: F, t28009: F, t5039: F, t92564: F, t28071: F, t3325: F, t15092: F, t7740: F, t10498: F, t3331: F, t8081: F) -> (F, F, F, F, F, F, F) {
    let t95483 = F::cast_from(4.0_f64) * t10491 * t28005;
    let t95485 = F::cast_from(4.0_f64) * t14668 * t26883;
    let t95487 = F::cast_from(4.0_f64) * t10491 * t28009;
    let t95489 = F::cast_from(4.0_f64) * t92564 * t5039;
    let t95491 = F::cast_from(2.0_f64) * t3325 * t28071;
    let t95492 = t7740 * t15092;
    let t95495 = F::cast_from(6.0_f64) * t10498 * t8081 * t3331;
    (t95483, t95485, t95487, t95489, t95491, t95492, t95495)
}
