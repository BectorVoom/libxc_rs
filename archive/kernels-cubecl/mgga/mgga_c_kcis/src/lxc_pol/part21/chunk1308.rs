//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1308/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1308<F: Float>(t1749: F, t303: F, t3183: F, t14413: F, t2173: F, t26857: F, t5329: F, t7709: F, t8034: F, t8042: F, t93006: F, t93008: F, t93010: F, t93012: F, t96042: F, t96045: F, t96048: F, t96052: F) -> (F, F) {
    let t96061 = t303 * t1749 * t3183;
    let t96063 = F::cast_from(0.69505208333333333333e-3_f64) * t2173 * t5329 * t7709 * t14413 - F::cast_from(0.37069444444444444444e-2_f64) * t26857 * t8034 + F::cast_from(0.13265555555555555555e-1_f64) * t96042 - F::cast_from(0.13265555555555555555e-1_f64) * t96045 - F::cast_from(0.22109259259259259258e-2_f64) * t96048 + F::cast_from(0.33163888888888888888e-2_f64) * t96052 - F::cast_from(0.37069444444444444444e-2_f64) * t26857 * t8042 + F::cast_from(0.33163888888888888888e-2_f64) * t93006 + F::cast_from(0.88437037037037037034e-2_f64) * t93008 - F::cast_from(0.33163888888888888888e-2_f64) * t93010 + F::cast_from(0.22109259259259259258e-2_f64) * t93012 - F::cast_from(0.88437037037037037034e-2_f64) * t96061;
    (t96061, t96063)
}
