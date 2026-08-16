//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 883/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk883<F: Float>(t6548: F, t8482: F, t322: F, t449: F, t9: F, t3105: F, t3109: F, t3103: F, t2855: F, t553: F, t1900: F) -> (F, F, F, F, F, F) {
    let t8483 = t8482 * t6548;
    let t8484 = t322 * t8483;
    let t8487 = t9 * t449;
    let t8488 = t8487 * t3105;
    let t8489 = t8488 * t3109;
    let t8490 = t3103 * t8489;
    let t8492 = t2855 * t553;
    let t8493 = t8492 * t1900;
    (t8483, t8484, t8487, t8488, t8490, t8493)
}
