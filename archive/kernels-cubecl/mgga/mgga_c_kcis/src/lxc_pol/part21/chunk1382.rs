//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1382/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1382<F: Float>(t187: F, t95514: F, t95517: F, t95520: F, t96542: F, t96545: F, t96668: F, t96689: F, t96708: F, t97499: F, t97500: F, t97501: F, t97503: F, t97505: F, t97507: F, t97510: F, t97511: F, t97513: F, t97517: F, t97521: F, t97526: F, t97528: F, t97529: F) -> F {
    let t97533 = t95514 + t95517 + t95520 + t96542 + t96545 + t187 * (t96668 + t96689 + t96708 + t97529) - t97499 - t97500 - t97501 - t97503 + t97505 + t97507 + t97510 - t97511 - t97513 + t97517 + t97521 - t97526 + t97528;
    t97533
}
