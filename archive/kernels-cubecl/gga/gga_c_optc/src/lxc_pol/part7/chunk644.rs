//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 644/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk644<F: Float>(t3235: F, t3237: F, t1148: F, t426: F, t911: F, t115: F, sigma2: F) -> (F, F, F, F) {
    let t3238 = t3235 * t3237;
    let t3241 = t1148 * sigma2;
    let t3242 = t426 * t911;
    let t3243 = t3242 * t115;
    let t3244 = t3241 * t3243;
    (t3238, t3241, t3242, t3244)
}
