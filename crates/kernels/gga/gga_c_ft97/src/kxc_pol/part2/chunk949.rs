//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 949/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk949<F: Float>(t1471: F, t4092: F, t1701: F, t213: F, t1109: F, t811: F, t820: F, t2725: F, t6: F, t285: F, t2726: F, t3780: F) -> (F, F, F, F, F) {
    let t14721 = t4092 * t1471;
    let t14722 = t1701 * t213;
    let t14723 = t1109 * t811;
    let t14724 = t14723 * t820;
    let t14725 = t14722 * t14724;
    let t14728 = t2725 * t6;
    let t14729 = t285 * t14728;
    let t14730 = t3780 * t2726;
    (t14721, t14725, t14728, t14729, t14730)
}
