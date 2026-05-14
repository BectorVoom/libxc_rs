//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 803/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk803<F: Float>(t1181: F, t3759: F, t604: F, t7426: F, t3073: F, t7646: F, t1530: F, t7560: F, t14046: F, t2067: F) -> (F, F, F, F) {
    let t30347 = t7426 * t1181 * t604 * t3759;
    let t30364 = t3073 * t7646;
    let t30371 = t1530 * t7560;
    let t30374 = t14046 * t2067;
    (t30347, t30364, t30371, t30374)
}
