//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1362/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1362<F: Float>(t1267: F, t15534: F, t167: F, t28110: F, t15239: F, t26960: F, t26961: F, t67957: F, t7788: F, t8087: F, t92613: F, t92896: F, t92898: F, t92908: F, t92921: F, t92929: F, t92931: F, t95989: F, t95992: F, t96714: F) -> (F, F) {
    let t97141 = t15534 * t28110 * t167 * t1267;
    let t97147 = -F::cast_from(0.69505208333333333334e-3_f64) * t7788 * t96714 - F::cast_from(0.15445601851851851852e-3_f64) * t92896 - F::cast_from(0.15445601851851851852e-3_f64) * t92898 - F::cast_from(0.41270617283950617284e-2_f64) * t92908 - F::cast_from(0.61782407407407407408e-3_f64) * t92921 - F::cast_from(0.15476481481481481481e-2_f64) * t92929 + F::cast_from(0.33980324074074074074e-2_f64) * t92613 * t8087 + F::cast_from(0.46336805555555555556e-3_f64) * t26960 * t15239 * t26961 * t67957 + F::cast_from(0.46336805555555555556e-3_f64) * t26960 * t97141 - F::cast_from(0.30918233506944444444e-4_f64) * t92931 + F::cast_from(0.61905925925925925926e-2_f64) * t95989 + F::cast_from(0.46429444444444444443e-2_f64) * t95992;
    (t97141, t97147)
}
