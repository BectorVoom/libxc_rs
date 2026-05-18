//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1094/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1094<F: Float>(t7839: F, t9593: F, t1165: F, t2068: F, t38837: F, t8600: F, t1089: F, t2090: F, t27338: F, t598: F, t30364: F, t6184: F) -> (F, F, F, F) {
    let t38899 = t7839 * t9593;
    let t38903 = t2068 * t1165 * t8600 * t38837;
    let t38909 = t598 * t1089 * t27338 * t2090;
    let t38912 = t30364 * t6184;
    (t38899, t38903, t38909, t38912)
}
