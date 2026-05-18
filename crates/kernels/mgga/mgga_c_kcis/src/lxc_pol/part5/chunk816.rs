//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 816/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk816<F: Float>(t304: F, t6480: F, t355: F, t360: F, t303: F, t1767: F) -> (F, F, F, F) {
    let t6481 = t304 * t6480;
    let t6482 = t6481 * t355;
    let t6483 = t6482 * t360;
    let t6484 = t303 * t6483;
    let t6486 = t1767 * t1767;
    (t6482, t6483, t6484, t6486)
}
