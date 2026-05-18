//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 762/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk762<F: Float>(t1561: F, t2885: F, t1574: F, t2838: F, t490: F, t1113: F, t23: F, t191: F, t24: F, t3086: F, t496: F, t8414: F) -> (F, F, F, F, F, F, F) {
    let t11700 = t1561 * t2885;
    let t11760 = t1574 * t2838;
    let t11761 = t490 * t11760;
    let t11781 = t23 * t1113;
    let t11782 = t11781 * t191;
    let t11885 = t24 * t3086;
    let t11894 = t496 * t8414;
    (t11700, t11760, t11761, t11781, t11782, t11885, t11894)
}
