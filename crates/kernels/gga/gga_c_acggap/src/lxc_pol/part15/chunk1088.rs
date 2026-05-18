//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1088/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1088<F: Float>(t38795: F, t7380: F, t4643: F, t8489: F, t2095: F, t1988: F, t9543: F, t1089: F, t3201: F, t598: F, t9541: F, t1083: F, t137: F, t5784: F) -> (F, F, F, F, F, F) {
    let t38796 = t7380 * t38795;
    let t38798 = t4643 * t8489;
    let t38799 = t2095 * t38798;
    let t38801 = t1988 * t9543;
    let t38805 = t598 * t1089 * t3201 * t9541;
    let t38810 = t598 * t1089 * t1083 * t137 * t5784;
    (t38796, t38798, t38799, t38801, t38805, t38810)
}
