//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 677/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk677<F: Float>(t2375: F, t678: F, t2366: F, t2372: F, t56: F, t649: F, t691: F, t2337: F, t28: F, t14: F, t2341: F, t8663: F, t4620: F, t4714: F, t8594: F, t8596: F, t8598: F, t8691: F, t8693: F, t8695: F) -> (F, F, F, F) {
    let t8712 = t2375 * t678;
    let t8713 = t2372 * t2366 * t8712;
    let t8717 = t649 * t691 * t56;
    let t8721 = 1.0 / t2337 / t28;
    let t8722 = t14 * t8721;
    let t8723 = t8663 * t2341;
    let t8725 = 0.96490945932906628932e2 * t8722 * t8723;
    let t8734 = -0.25319e1 * t8594 + 0.16879333333333333333e1 * t8596 - 0.19692555555555555555e1 * t8598 - 0.93011851851851851854e0 * t4620 + 0.13651666666666666667e0 * t8691 - 0.27303333333333333333e0 * t8693 - 0.3185388888888888889e0 * t8695 - 0.36514074074074074075e0 * t4714;
    (t8713, t8717, t8725, t8734)
}
