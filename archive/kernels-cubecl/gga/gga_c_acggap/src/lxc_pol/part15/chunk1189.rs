//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1189/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1189<F: Float>(t336: F, t5674: F, t578: F, t599: F, t1773: F, t2060: F, t2061: F, t6388: F, t7450: F, t7815: F, t13299: F, t33952: F, t40440: F) -> (F, F, F, F) {
    let t40573 = t578 * t336 * t599 * t5674;
    let t40576 = t2060 * t1773 * t2061;
    let t40579 = t7450 * t7815 * t6388;
    let t40584 = t33952 * t13299 * t40440;
    (t40573, t40576, t40579, t40584)
}
