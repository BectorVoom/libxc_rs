//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 642/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk642<F: Float>(t1477: F, t3809: F, t542: F, t1409: F, t543: F, t1419: F) -> (F, F, F, F) {
    let t3810 = t1477 * t3809;
    let t3811 = t542 * t3810;
    let t3814 = t543 * t1409;
    let t3815 = t1419 * t1419;
    (t3810, t3811, t3814, t3815)
}
