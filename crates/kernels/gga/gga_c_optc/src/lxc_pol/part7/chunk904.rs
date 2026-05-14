//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 904/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk904<F: Float>(t8593: F, t8595: F, t8598: F, t8601: F, t8603: F, t8606: F, t8609: F, t8651: F, t8654: F, t8657: F, t8660: F, t9322: F, t1200: F, t1205: F, t2881: F, t2886: F, t2887: F, t2900: F, t485: F, t9292: F, t9294: F, t9297: F, t9304: F, t9305: F, t9308: F) -> (F, F) {
    let t9334 = 0.48461111111111111112e3 * t8593 - 0.31466666666666666667e3 * t8595 + 0.15733333333333333333e3 * t8598 - 0.78666666666666666666e2 * t8651 - 0.47199999999999999999e3 * t8601 + 0.47199999999999999999e3 * t8654 - 0.14538333333333333333e4 * t8603 + 0.29076666666666666666e4 * t8606 - 0.14538333333333333333e4 * t8657 - 0.43614999999999999999e4 * t8609 + 0.43614999999999999999e4 * t8660;
    let t9335 = t9322 + t9334;
    let t9337 = -t1200 * t9335 - 3.0 * t9294 * t1205 - 3.0 * t2881 * t2900 + 6.0 * t2886 * t9308 + 6.0 * t9297 * t2887 + t9292 * t485 - 6.0 * t9304 * t9305;
    (t9335, t9337)
}
