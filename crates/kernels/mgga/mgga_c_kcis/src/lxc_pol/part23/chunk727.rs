//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 727/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk727<F: Float>(t2337: F, t28: F, t14: F, t2341: F, t8663: F, t4620: F, t4714: F, t8594: F, t8596: F, t8598: F, t8691: F, t8693: F, t8695: F) -> (F, F) {
    let t8721 = F::cast_from(1.0_f64) / t2337 / t28;
    let t8722 = t14 * t8721;
    let t8723 = t8663 * t2341;
    let t8725 = F::cast_from(0.96490945932906628932e2_f64) * t8722 * t8723;
    let t8734 = -F::cast_from(0.25319e1_f64) * t8594 + F::cast_from(0.16879333333333333333e1_f64) * t8596 - F::cast_from(0.19692555555555555555e1_f64) * t8598 - F::cast_from(0.93011851851851851854e0_f64) * t4620 + F::cast_from(0.13651666666666666667e0_f64) * t8691 - F::cast_from(0.27303333333333333333e0_f64) * t8693 - F::cast_from(0.3185388888888888889e0_f64) * t8695 - F::cast_from(0.36514074074074074075e0_f64) * t4714;
    (t8725, t8734)
}
