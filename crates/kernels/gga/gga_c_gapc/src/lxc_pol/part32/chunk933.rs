//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 933/1311 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk933<F: Float>(t11428: F, t1461: F, t1030: F, t8716: F, t129: F, t5541: F, t3021: F, t5544: F, t5462: F, t5549: F, t11387: F, t1649: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t11429 = t11428 * M_PI;
    let t11430 = t1461 * t11429;
    let t11431 = t1030 * t11430;
    let t11432 = t11431 * t8716;
    let t11434 = t5541 * t129;
    let t11435 = t3021 * t5544;
    let t11436 = t11434 * t11435;
    let t11438 = t5462 * t129;
    let t11439 = t3021 * t5549;
    let t11440 = t11438 * t11439;
    let t11442 = t11387 * t1649;
    (t11430, t11431, t11432, t11434, t11435, t11436, t11438, t11439, t11440, t11442)
}
