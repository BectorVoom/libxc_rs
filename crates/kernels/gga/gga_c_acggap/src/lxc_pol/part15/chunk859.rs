//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 859/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk859<F: Float>(t1165: F, t5249: F, t604: F, t7493: F, t7433: F, t8869: F, t7839: F, t1411: F, t1992: F, t7585: F, t7842: F, t31699: F, t8665: F, t30409: F, t30418: F, t31309: F, t525: F) -> (F, F, F, F, F, F) {
    let t33839 = t7493 * t1165 * t604 * t5249;
    let t33841 = t7433 * t8869;
    let t33843 = t7839 * t8869;
    let t33851 = t7585 * t7842 * t1992 * t1411;
    let t33853 = t31699 * t8665;
    let t33857 = t31309 * t30418 * t30409 * t525;
    (t33839, t33841, t33843, t33851, t33853, t33857)
}
