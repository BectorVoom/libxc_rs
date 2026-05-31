//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 497/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk497<F: Float>(t1122: F, t3178: F, t1092: F, t1130: F, t982: F, t1133: F, t1021: F, t89: F, t828: F, t2635: F, t8: F) -> (F, F, F, F, F, F, F) {
    let t3179 = t3178 * t1122;
    let t3180 = t1092 * t3179;
    let t3182 = t982 * t1130;
    let t3183 = t3182 * t1133;
    let t3184 = t1021 * t3183;
    let t3185 = t1092 * t3184;
    let t3187 = F::cast_from(2.0_f64) * t89;
    let t3188 = F::cast_from(2.0_f64) * t828;
    let t3190 = t2635 * t8 + t3187 - t3188;
    (t3179, t3180, t3182, t3183, t3184, t3185, t3190)
}
