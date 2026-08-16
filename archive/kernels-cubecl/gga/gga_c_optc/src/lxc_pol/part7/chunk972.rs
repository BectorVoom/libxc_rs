//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 972/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk972<F: Float>(t275: F, t9337: F, t176: F, t498: F, t8560: F, t8564: F, t8574: F, t8703: F, t8705: F, t8707: F, t8745: F, t8747: F, t8753: F, t8898: F, t9266: F, sigma2: F) -> (F, F) {
    let t9338 = t9337 * t275;
    let t9340 = t176 * t9338 * sigma2;
    let t9343 = -t8560 + t8564 + t8574 - t8703 - t8705 - t8707 + t9266 / F::cast_from(2.0_f64) + t9340 * t498 / F::cast_from(2.0_f64) - t8745 + t8747 + t8753 - t8898;
    (t9340, t9343)
}
