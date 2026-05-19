//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1250/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1250<F: Float>(t1020: F, t8047: F, t95893: F, t2822: F, t28908: F, t100383: F, t100386: F, t100389: F, t100398: F, t15227: F, t19396: F, t20552: F, t26955: F, t26960: F, t5329: F, t7788: F, t7794: F, t96754: F, t96942: F, t96943: F, t96945: F) -> (F, F, F) {
    let t100401 = t1020 * t95893 * t8047;
    let t100407 = t2822 * t28908;
    let t100409 = F::cast_from(0.69644166666666666664e-2_f64) * t100383 - F::cast_from(0.77382407407407407407e-3_f64) * t100386 + F::cast_from(0.20612155671296296296e-4_f64) * t26955 * t100389 - F::cast_from(0.36039737654320987655e-3_f64) * t26960 * t15227 * t96754 * t19396 + t96942 + F::cast_from(0.69644166666666666664e-2_f64) * t100398 + F::cast_from(0.23214722222222222222e-2_f64) * t100401 + t96943 + F::cast_from(0.34752604166666666667e-3_f64) * t7788 * t5329 * t7794 * t20552 + F::cast_from(0.77382407407407407407e-3_f64) * t100407 + t96945;
    (t100401, t100407, t100409)
}
