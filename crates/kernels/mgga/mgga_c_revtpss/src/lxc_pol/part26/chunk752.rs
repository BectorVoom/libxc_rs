//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 752/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk752<F: Float>(t10535: F, t10538: F, t2783: F, t860: F, t786: F, t2801: F, t231: F, t2645: F, t268: F, t675: F, t2798: F, t10430: F, t10432: F, t10435: F, t10438: F, t10442: F, t10444: F, t10469: F, t9278: F, t9308: F, t9316: F, t9329: F) -> (F, F, F, F) {
    let t10539 = t10535 * t10538;
    let t10541 = t2783 * t860;
    let t10542 = t786 * t10541;
    let t10543 = t10542 * t2801;
    let t10547 = t268 * t675 * t2645 * t231;
    let t10548 = t2798 * t10547;
    let t10550 = t10430 + t10432 + t10435 + t10438 + t10442 - t9278 + t9308 + t9316 + t10444 + t10469 + t9329;
    (t10539, t10543, t10548, t10550)
}
