//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 214/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk214<F: Float>(t601: F, t603: F, t103: F, t172: F, t47: F, t52: F, t37: F, t7: F, t8: F) -> (F, F, F, F, F, F) {
    let t605 = F::new(0.58482233974552040708e0) * t601 * t603;
    let t606 = t103 * t172;
    let t607 = F::new(1.0) / t47;
    let t611 = F::new(1.0) / t52;
    let t620 = t37 * t7;
    let t622 = F::new(1.0) / t8 / t620;
    (t605, t606, t607, t611, t620, t622)
}
