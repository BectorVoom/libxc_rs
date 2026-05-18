//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 982/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk982<F: Float>(t17978: F, t3133: F, t9171: F, t4477: F, t5096: F, t3245: F, t17855: F, t429: F, t438: F, t914: F, t1442: F, t9073: F) -> (F, F, F, F, F, F, F) {
    let t17979 = t17978 * t3133;
    let t17982 = t17978 * t9171;
    let t17987 = t4477 * t5096;
    let t17988 = t3245 * t17987;
    let t17993 = t429 * t17855 * t438;
    let t17994 = t914 * t17993;
    let t18005 = t9073 * t1442;
    (t17979, t17982, t17987, t17988, t17993, t17994, t18005)
}
