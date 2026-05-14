//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 676/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk676<F: Float>(t1396: F, t4124: F, t4123: F, t1464: F, t1489: F, t1497: F) -> (F, F, F, F) {
    let t4125 = t1396 * t4124;
    let t4126 = t4123 * t4125;
    let t4127 = t1464 * t4126;
    let t4129 = t1489 * t1497;
    (t4125, t4126, t4127, t4129)
}
