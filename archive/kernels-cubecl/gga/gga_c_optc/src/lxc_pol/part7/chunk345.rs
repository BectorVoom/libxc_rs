//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 345/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk345<F: Float>(t1115: F, t322: F, t446: F, t871: F, t140: F, t464: F) -> (F, F, F, F) {
    let t1116 = t322 * t1115;
    let t1119 = t446 * t871;
    let t1120 = t1119 * t140;
    let t1121 = t464 * t1120;
    (t1116, t1119, t1120, t1121)
}
