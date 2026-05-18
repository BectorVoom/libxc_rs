//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 664/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk664<F: Float>(t2672: F, t875: F, t1: F, t2769: F, t282: F, t3883: F) -> (F, F, F, F) {
    let t3908 = t2672 * t875;
    let t3909 = t3908 * t1;
    let t3916 = t2769 * t282;
    let t3917 = t3916 * t3883;
    (t3908, t3909, t3916, t3917)
}
