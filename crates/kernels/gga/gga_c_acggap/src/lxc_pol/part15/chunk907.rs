//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 907/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk907<F: Float>(t7637: F, t7796: F, t1004: F, t390: F, t7613: F, t151: F, t37: F, t56: F, t593: F, t7508: F, t141: F, t420: F) -> (F, F, F, F) {
    let t30993 = t7637 * t7796;
    let t31001 = t1004 * t7613 * t390;
    let t31009 = t151 * t593 / t7508 / t37 * t56;
    let t31010 = t420 * t141;
    (t30993, t31001, t31009, t31010)
}
