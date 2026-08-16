//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1346/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1346<F: Float>(t15579: F, t5329: F, t7773: F, t15553: F, t28145: F, t7788: F, t26966: F, t27042: F, t28132: F, t28137: F, t28184: F, t95736: F, t95739: F, t95742: F, t95745: F, t95748: F) -> (F, F) {
    let t96831 = t5329 * t7773 * t15579;
    let t96836 = t7788 * t15553 * t28145;
    let t96846 = F::cast_from(0.18534722222222222222e-2_f64) * t26966 * t28132 + F::cast_from(0.37069444444444444444e-2_f64) * t26966 * t28137 + F::cast_from(0.34752604166666666667e-3_f64) * t7788 * t96831 + F::cast_from(0.69644166666666666664e-2_f64) * t95736 + F::cast_from(0.54059606481481481482e-3_f64) * t96836 + F::cast_from(0.18534722222222222222e-2_f64) * t26966 * t28184 + F::cast_from(0.24734586805555555556e-3_f64) * t27042 * t28184 - F::cast_from(0.41270617283950617284e-2_f64) * t95739 + F::cast_from(0.12381185185185185185e-1_f64) * t95742 - F::cast_from(0.10317654320987654321e-1_f64) * t95745 + F::cast_from(0.15476481481481481481e-2_f64) * t95748;
    (t96831, t96846)
}
