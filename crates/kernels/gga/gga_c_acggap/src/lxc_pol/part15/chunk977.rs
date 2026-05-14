//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 977/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk977<F: Float>(t6396: F, t7822: F, t6400: F, t30148: F, t6841: F, t7585: F, t7842: F, t1165: F, t38883: F, t604: F, t7337: F, t1181: F, t1844: F, t2068: F, t360: F, t30268: F, t9589: F) -> (F, F, F, F, F, F) {
    let t39145 = t7822 * t6396;
    let t39147 = t7822 * t6400;
    let t39151 = t7585 * t7842 * t30148 * t6841;
    let t39155 = t7337 * t1165 * t604 * t38883;
    let t39160 = t2068 * t1181 * t604 * t1844 * t360;
    let t39162 = t30268 * t9589;
    (t39145, t39147, t39151, t39155, t39160, t39162)
}
