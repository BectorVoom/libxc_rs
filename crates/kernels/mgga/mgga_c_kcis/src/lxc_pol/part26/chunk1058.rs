//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1058/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1058<F: Float>(t7497: F, t7969: F, t6176: F, t27596: F, t7509: F) -> (F, F, F, F) {
    let t29509 = t7969 * t7497;
    let t29510 = t6176 * t29509;
    let t29513 = t27596 * t7509;
    let t29514 = t6176 * t29513;
    (t29509, t29510, t29513, t29514)
}
