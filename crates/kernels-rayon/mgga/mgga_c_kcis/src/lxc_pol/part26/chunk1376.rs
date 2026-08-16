//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1376/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1376(t18210: f64, t29299: f64, t2237: f64, t17287: f64, t6140: f64, t102563: f64, t102586: f64, t103502: f64, t103507: f64, t16744: f64, t2239: f64, t29324: f64, t29407: f64, t7895: f64, t7901: f64, t7916: f64, t8148: f64, t94227: f64, t98392: f64, t98719: f64, t98721: f64) -> (f64, f64) {
    let t103626 = t18210 * t29299;
    let t103627 = t2237 * t103626;
    let t103646 = t17287 * t6140;
    let t103649 = -0.24872916666666666666e-2_f64 * t102563 + 0.23168402777777777778e-3_f64 * t103627 - 0.13901041666666666667e-2_f64 * t7895 * t29324 - 0.12378114784505208333e-4_f64 * t98392 * t103502 + 0.82448622685185185185e-4_f64 * t94227 * t103507 - 0.7369753086419753086e-3_f64 * t102586 - 0.4946917361111111111e-3_f64 * t98721 * t8148 - 0.11054629629629629629e-2_f64 * t98719 + 0.37069444444444444444e-2_f64 * t16744 * t6140 * t2239 - 0.37069444444444444444e-2_f64 * t29407 * t7916 - 0.37069444444444444444e-2_f64 * t29407 * t7901 - 0.49469173611111111111e-3_f64 * t103646 * t7901;
    (t103626, t103649)
}
