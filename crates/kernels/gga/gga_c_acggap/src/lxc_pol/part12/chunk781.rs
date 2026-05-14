//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 781/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk781<F: Float>(t1416: F, t372: F, t1345: F, t322: F, t1662: F, t301: F, t467: F, t495: F, t811: F, t7884: F, t7911: F, t7930: F, t862: F, t309: F, t871: F, t1210: F, t618: F) -> (F, F, F, F, F, F, F, F, F) {
    let t23736 = t1416 * t372;
    let t23745 = t1345 * t322;
    let t24589 = t301 * t1662;
    let t24605 = t1662 * t467;
    let t24623 = t495 * t811;
    let t29976 = t7884 * t7911;
    let t29979 = t862 * t7930;
    let t29980 = t871 * t309;
    let t29984 = t1210 * t618;
    (t23736, t23745, t24589, t24605, t24623, t29976, t29979, t29980, t29984)
}
