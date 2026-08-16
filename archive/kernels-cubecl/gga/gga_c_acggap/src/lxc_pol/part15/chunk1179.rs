//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1179/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1179<F: Float>(t1165: F, t5697: F, t7575: F, t8600: F, t1795: F, t1983: F, t2095: F, t1967: F, t9577: F, t1426: F, t2085: F, t22099: F, t598: F) -> (F, F, F, F) {
    let t40418 = t7575 * t1165 * t8600 * t5697;
    let t40425 = t2095 * t1983 * t1795;
    let t40427 = t1967 * t9577;
    let t40431 = t598 * t1426 * t22099 * t2085;
    (t40418, t40425, t40427, t40431)
}
