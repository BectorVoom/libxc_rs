//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 965/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk965<F: Float>(t5864: F, t7822: F, t5869: F, t5895: F, t5899: F, t1844: F, t1983: F, t7585: F, t7586: F, t1750: F, t30948: F, t1165: F, t30806: F, t5824: F, t604: F, t7433: F, t9641: F) -> (F, F, F, F, F, F, F, F) {
    let t39080 = t7822 * t5864;
    let t39082 = t7822 * t5869;
    let t39086 = t7822 * t5895;
    let t39088 = t7822 * t5899;
    let t39092 = t7585 * t7586 * t1983 * t1844;
    let t39094 = t30948 * t1750;
    let t39098 = t30806 * t1165 * t604 * t5824;
    let t39100 = t7433 * t9641;
    (t39080, t39082, t39086, t39088, t39092, t39094, t39098, t39100)
}
