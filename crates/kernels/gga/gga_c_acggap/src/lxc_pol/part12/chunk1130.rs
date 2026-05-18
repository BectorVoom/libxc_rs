//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1130/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1130<F: Float>(t1181: F, t4623: F, t604: F, t7426: F, t30090: F, t8897: F, t31362: F, t8903: F, t7839: F, t8908: F, t8912: F, t1165: F, t2068: F, t35102: F, t7351: F) -> (F, F, F, F, F, F) {
    let t36081 = t7426 * t1181 * t604 * t4623;
    let t36083 = t30090 * t8897;
    let t36085 = t31362 * t8903;
    let t36087 = t7839 * t8908;
    let t36089 = t7839 * t8912;
    let t36093 = t2068 * t1165 * t7351 * t35102;
    (t36081, t36083, t36085, t36087, t36089, t36093)
}
