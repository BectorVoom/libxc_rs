//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 962/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk962<F: Float>(t11603: F, t3429: F, t2816: F, t3446: F, t3453: F, t1102: F, t3314: F, t3692: F, t3582: F, t792: F, t6967: F, t795: F) -> (F, F, F, F, F) {
    let t11604 = t3429 * t11603;
    let t11607 = t3446 * t3453 * t2816;
    let t11616 = t1102 * t3314 * t3692;
    let t11621 = t3582 * t792;
    let t11625 = t6967 * t795;
    (t11604, t11607, t11616, t11621, t11625)
}
