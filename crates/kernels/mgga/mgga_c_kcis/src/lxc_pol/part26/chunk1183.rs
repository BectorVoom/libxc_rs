//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1183/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1183<F: Float>(t4413: F, t6136: F, t12857: F, t2093: F, t4188: F, t7267: F, t1505: F, t22298: F, t38630: F, t7042: F, t12321: F, t6922: F) -> (F, F, F, F, F, F) {
    let t54605 = t6136 * t4413;
    let t54624 = t2093 * t12857;
    let t54732 = t7267 * t4188;
    let t54773 = t22298 * t1505;
    let t58540 = t7042 * t38630;
    let t58599 = t12321 * t6922;
    (t54605, t54624, t54732, t54773, t58540, t58599)
}
