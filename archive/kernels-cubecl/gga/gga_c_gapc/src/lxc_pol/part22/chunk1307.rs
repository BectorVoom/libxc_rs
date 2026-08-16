//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1307/1426 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1307<F: Float>(t11261: F, t13337: F, t1416: F, t1672: F, t1: F, t516: F, t619: F, t6803: F, t8379: F, t2941: F, t3638: F, t3954: F) -> (F, F, F) {
    let t35689 = t11261 * t1416 * t1672 * t13337;
    let t35694 = t8379 * t516 * t1 * t6803 * t619;
    let t35697 = t2941 * t3638 * t3954;
    (t35689, t35694, t35697)
}
