//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1410/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1410<F: Float>(t24229: F, t34321: F, t24235: F, t34368: F, t112102: F, t24226: F, t24275: F, t33121: F, t24177: F, t9708: F, t122337: F, t122339: F, t122341: F, t122344: F, t122347: F, t122349: F) -> (F, F, F, F, F, F) {
    let t122351 = t34321 * t24229;
    let t122353 = t34368 * t24235;
    let t122355 = t112102 * t24226;
    let t122357 = t33121 * t24275;
    let t122359 = t9708 * t24177;
    let t122361 = -11.0 / 18.0 * t122337 + t122339 / 9.0 - t122341 / 72.0 + t122344 / 24.0 - t122347 / 144.0 - t122349 / 8.0 - t122351 / 288.0 - t122353 / 32.0 - 3.0 / 8.0 * t122355 + t122357 / 144.0 - t122359 / 72.0;
    (t122351, t122353, t122355, t122357, t122359, t122361)
}
