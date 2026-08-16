//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1376/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1376<F: Float>(t18210: F, t29299: F, t2237: F, t17287: F, t6140: F, t102563: F, t102586: F, t103502: F, t103507: F, t16744: F, t2239: F, t29324: F, t29407: F, t7895: F, t7901: F, t7916: F, t8148: F, t94227: F, t98392: F, t98719: F, t98721: F) -> (F, F) {
    let t103626 = t18210 * t29299;
    let t103627 = t2237 * t103626;
    let t103646 = t17287 * t6140;
    let t103649 = -F::cast_from(0.24872916666666666666e-2_f64) * t102563 + F::cast_from(0.23168402777777777778e-3_f64) * t103627 - F::cast_from(0.13901041666666666667e-2_f64) * t7895 * t29324 - F::cast_from(0.12378114784505208333e-4_f64) * t98392 * t103502 + F::cast_from(0.82448622685185185185e-4_f64) * t94227 * t103507 - F::cast_from(0.7369753086419753086e-3_f64) * t102586 - F::cast_from(0.4946917361111111111e-3_f64) * t98721 * t8148 - F::cast_from(0.11054629629629629629e-2_f64) * t98719 + F::cast_from(0.37069444444444444444e-2_f64) * t16744 * t6140 * t2239 - F::cast_from(0.37069444444444444444e-2_f64) * t29407 * t7916 - F::cast_from(0.37069444444444444444e-2_f64) * t29407 * t7901 - F::cast_from(0.49469173611111111111e-3_f64) * t103646 * t7901;
    (t103626, t103649)
}
