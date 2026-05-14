//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 614/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk614<F: Float>(t1207: F, t1211: F, t1210: F, t401: F, t396: F, t1219: F) -> (F, F, F, F) {
    let t3545 = t1207 * t1211;
    let t3548 = t1210 * t401;
    let t3549 = 1.0 / t3548;
    let t3550 = t396 * t3549;
    let t3551 = t1219 * t1219;
    (t3545, t3549, t3550, t3551)
}
