//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1150/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1150<F: Float>(t27924: F, t303: F, t3233: F, t13174: F, t4994: F, t7718: F, t1020: F, t13137: F, t10470: F, t13113: F, t3198: F, t355: F, t1749: F, t3183: F, t14413: F, t2173: F, t26857: F, t5329: F, t7709: F, t8034: F, t8042: F, t93006: F, t93008: F, t93010: F, t93012: F) -> (F, F, F, F, F, F) {
    let t96042 = t303 * t27924 * t3233;
    let t96045 = t4994 * t7718 * t13174;
    let t96048 = t1020 * t7718 * t13137;
    let t96052 = t10470 * t3198 * t355 * t13113;
    let t96061 = t303 * t1749 * t3183;
    let t96063 = 0.69505208333333333333e-3 * t2173 * t5329 * t7709 * t14413 - 0.37069444444444444444e-2 * t26857 * t8034 + 0.13265555555555555555e-1 * t96042 - 0.13265555555555555555e-1 * t96045 - 0.22109259259259259258e-2 * t96048 + 0.33163888888888888888e-2 * t96052 - 0.37069444444444444444e-2 * t26857 * t8042 + 0.33163888888888888888e-2 * t93006 + 0.88437037037037037034e-2 * t93008 - 0.33163888888888888888e-2 * t93010 + 0.22109259259259259258e-2 * t93012 - 0.88437037037037037034e-2 * t96061;
    (t96042, t96045, t96048, t96052, t96061, t96063)
}
