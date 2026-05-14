//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 747/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk747<F: Float>(t1891: F, t875: F, t2643: F, t3821: F, t22: F, t2595: F, t2263: F, t1885: F, t2639: F, t946: F) -> (F, F, F, F, F, F, F) {
    let t7476 = t1891 * t875;
    let t7477 = t7476 * t2643;
    let t7478 = t3821 * t7477;
    let t7481 = t22 * t2595;
    let t7482 = t7481 * t2263;
    let t7483 = t1885 * t875;
    let t7484 = t7483 * t2643;
    let t7485 = t7482 * t7484;
    let t7488 = t946 * t2639;
    (t7477, t7478, t7481, t7482, t7484, t7485, t7488)
}
