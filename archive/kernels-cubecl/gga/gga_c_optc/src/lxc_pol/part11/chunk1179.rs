//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1179/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1179<F: Float>(t1582: F, t1583: F, t17648: F, t6: F, t14863: F, t4230: F, t16094: F, t4536: F, t15178: F, t18194: F, t4215: F, t15104: F, t18200: F) -> (F, F, F, F, F, F, F) {
    let t53399 = t1582 * t1583 * t17648 * t6;
    let t53432 = t4230 * t14863;
    let t53443 = t4536 * t16094;
    let t53445 = t4230 * t15178;
    let t53453 = t4536 * t15178;
    let t53465 = t18194 * t4215;
    let t53470 = t18200 * t15104;
    (t53399, t53432, t53443, t53445, t53453, t53465, t53470)
}
