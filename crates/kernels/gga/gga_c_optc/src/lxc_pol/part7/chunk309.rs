//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 309/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk309<F: Float>(t115: F, t852: F, t5: F, t363: F, t362: F, t857: F, t357: F, t355: F, t176: F, t352: F) -> (F, F, F, F, F, F, F) {
    let t987 = t852 * t115;
    let t988 = t987 * t5;
    let t989 = t988 * t363;
    let t992 = t857 * t362;
    let t993 = t357 * t992;
    let t995 = t355 * t993 / F::new(6.0);
    let t996 = t176 * t352;
    (t987, t988, t989, t992, t993, t995, t996)
}
