//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 652/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk652<F: Float>(t130: F, t635: F, t140: F, t2086: F, t106: F, t145: F, t146: F, t692: F, t112: F) -> (F, F, F, F, F) {
    let t3439 = t130 * t635;
    let t3440 = t2086 * t140;
    let t3461 = t106 * t145;
    let t3466 = t146 * t692;
    let t3467 = t3466 * t112;
    (t3439, t3440, t3461, t3466, t3467)
}
