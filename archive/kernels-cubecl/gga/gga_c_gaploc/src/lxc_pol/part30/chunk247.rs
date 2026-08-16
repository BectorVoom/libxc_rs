//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 247/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk247<F: Float>(t898: F, t901: F, t531: F, t888: F, t569: F, t874: F, t568: F, t169: F, t78: F) -> (F, F, F, F, F) {
    let t902 = t898 * t901;
    let t904 = t531 * t888;
    let t907 = t569 * t874;
    let t908 = t568 * t907;
    let t911 = t78 * t169;
    (t902, t904, t907, t908, t911)
}
