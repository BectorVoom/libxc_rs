//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1235/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1235<F: Float>(t7906: F, t909: F, t911: F, t2742: F, t2773: F, t2774: F, t2778: F, t2780: F, t115: F, t2341: F, t2770: F, t2769: F) -> (F, F, F, F, F) {
    let t25504 = t909 * t7906 * t911;
    let t25508 = t2773 * t2742 * t2774;
    let t25511 = t2778 * t2742 * t2780;
    let t25514 = t2341 * t2770 * t115;
    let t25515 = t2769 * t25514;
    (t25504, t25508, t25511, t25514, t25515)
}
