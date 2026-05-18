//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 354/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk354<F: Float>(t1115: F, t914: F, t439: F, t462: F, t935: F, t447: F, t871: F) -> (F, F, F, F) {
    let t1163 = t914 * t1115;
    let t1167 = F::new(1.0) / t462 / t439;
    let t1168 = t935 * t1167;
    let t1170 = t1168 * t447 * t871;
    (t1163, t1167, t1168, t1170)
}
