//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1108/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1108<F: Float>(t1165: F, t2068: F, t25742: F, t7351: F, t6271: F, t7561: F, t6396: F, t7822: F, t6400: F, t30148: F, t6841: F, t7585: F, t7842: F) -> (F, F, F, F, F) {
    let t39141 = t2068 * t1165 * t7351 * t25742;
    let t39143 = t7561 * t6271;
    let t39145 = t7822 * t6396;
    let t39147 = t7822 * t6400;
    let t39151 = t7585 * t7842 * t30148 * t6841;
    (t39141, t39143, t39145, t39147, t39151)
}
