//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1176/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1176<F: Float>(t1137: F, t5910: F, t1163: F, t1181: F, t1532: F, t1753: F, t879: F, t14176: F, t5732: F, t506: F, t955: F, t3431: F, t5712: F) -> (F, F, F, F, F) {
    let t21331 = t1137 * t5910;
    let t21338 = t1163 * t1181 * t1532 * t1753 * t879;
    let t21340 = t14176 * t5732;
    let t21342 = t955 * t506;
    let t21348 = t3431 * t5712;
    (t21331, t21338, t21340, t21342, t21348)
}
