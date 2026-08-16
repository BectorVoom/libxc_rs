//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1259/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1259<F: Float>(t5073: F, t92437: F, t14700: F, t7748: F, t15082: F, t26896: F, t26929: F, t5177: F, t9531: F, t380: F, t5182: F, t92514: F) -> (F, F, F, F, F) {
    let t95336 = t92437 * t5073;
    let t95338 = t7748 * t14700;
    let t95340 = t26896 * t15082;
    let t95343 = t9531 * t26929 * t5177;
    let t95346 = t380 * t92514 * t5182;
    (t95336, t95338, t95340, t95343, t95346)
}
