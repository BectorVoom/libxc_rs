//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1120/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1120<F: Float>(t16295: F, t732: F, t16247: F, t40: F, t591: F, t16248: F, t539: F, t544: F, t1: F, t598: F, t193: F, t39009: F, t4752: F) -> (F, F, F, F, F, F) {
    let t47765 = t732 * t16295;
    let t47871 = t40 * t16247 * t591;
    let t47877 = t539 * t16248;
    let t47879 = t544 * t16248;
    let t47886 = t16247 * t1 * t598;
    let t47896 = t193 * t39009 * t4752;
    (t47765, t47871, t47877, t47879, t47886, t47896)
}
