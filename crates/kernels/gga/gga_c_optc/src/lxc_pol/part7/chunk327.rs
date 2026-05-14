//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 327/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk327<F: Float>(t1030: F, t1041: F, t1043: F, t1046: F, t1050: F, t1053: F) -> (F,) {
    let t1055 = 0.1898925e1 * t1041 - t1043 - 0.29896666666666666667e0 * t1030 + 0.3071625e0 * t1046 - t1050 - 0.82156666666666666667e-1 * t1053;
    (t1055,)
}
