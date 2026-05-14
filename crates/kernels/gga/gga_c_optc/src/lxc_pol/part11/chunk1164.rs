//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1164/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1164<F: Float>(t16729: F, t3665: F, t57099: F, t780: F, t39565: F, t49404: F, t49406: F, t57057: F, t57060: F, t57063: F, t57066: F, t57069: F, t57071: F, t57073: F, t57100: F, t57102: F) -> (F, F, F) {
    let t57104 = t3665 * t16729;
    let t57106 = t780 * t57099;
    let t57108 = 0.10954222222222222222e1 * t39565 + 0.13145066666666666666e1 * t49404 - 0.43816888888888888888e0 * t49406 - 0.29896666666666666667e0 * t57057 + 0.71752e1 * t57060 + 0.17938e1 * t57063 + 0.46074375e0 * t57066 + 0.1151859375e0 * t57069 - 0.28483875e1 * t57071 - 0.3560484375e1 * t57073 + 0.3071625e0 * t57100 + 0.85451625e1 * t57102 - 0.379785e1 * t57104 + 0.1898925e1 * t57106;
    (t57104, t57106, t57108)
}
