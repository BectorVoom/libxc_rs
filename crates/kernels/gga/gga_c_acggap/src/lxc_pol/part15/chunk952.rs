//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 952/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk952<F: Float>(t315: F, t33795: F, t2137: F, t33428: F, t1432: F, t30147: F, t30148: F, t7842: F, t1165: F, t5249: F, t604: F, t7493: F) -> (F, F, F, F, F) {
    let t33796 = t315 * t33795;
    let t33799 = t2137 * t33795;
    let t33802 = t315 * t33428;
    let t33831 = t30147 * t7842 * t30148 * t1432;
    let t33839 = t7493 * t1165 * t604 * t5249;
    (t33796, t33799, t33802, t33831, t33839)
}
