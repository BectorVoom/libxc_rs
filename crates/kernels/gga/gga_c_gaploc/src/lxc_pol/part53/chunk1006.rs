//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 1006/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk1006<F: Float>(t1: F, t47008: F, t1415: F, t2413: F, t13829: F, t1646: F, t528: F, t13818: F, t1599: F, t46953: F, t531: F, t557: F) -> (F, F, F, F, F) {
    let t48086 = t47008 * t1;
    let t48087 = t1415 * t48086;
    let t48088 = t48087 * t2413;
    let t48093 = F::new(0.35750489951850426669e0) * t528 * t13829 * t1646;
    let t48096 = F::new(0.35750489951850426669e0) * t1599 * t13818;
    let t48099 = F::new(0.35750489951850426669e0) * t557 * t531 * t46953;
    (t48086, t48088, t48093, t48096, t48099)
}
