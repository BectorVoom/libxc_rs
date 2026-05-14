//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1020/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1020<F: Float>(t11927: F, t4536: F, t1220: F, t5102: F, t7274: F, t16287: F, t172: F, t16295: F, t732: F, t16247: F, t40: F, t591: F, t16248: F, t539: F, t544: F, t1: F, t598: F) -> (F, F, F, F, F, F, F, F) {
    let t47659 = t4536 * t11927;
    let t47709 = t1220 * t7274 * t5102;
    let t47744 = t172 * t16287;
    let t47765 = t732 * t16295;
    let t47871 = t40 * t16247 * t591;
    let t47877 = t539 * t16248;
    let t47879 = t544 * t16248;
    let t47886 = t16247 * t1 * t598;
    (t47659, t47709, t47744, t47765, t47871, t47877, t47879, t47886)
}
