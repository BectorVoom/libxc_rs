//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1159/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1159<F: Float>(t1468: F, t7305: F, t570: F, t7386: F, t28610: F, t8196: F, t6028: F, t7257: F, t7948: F, t28589: F, t8191: F, t5909: F, t6922: F) -> (F, F, F, F, F, F, F) {
    let t29459 = t1468 * t7305;
    let t29461 = t570 * t7386;
    let t29463 = t28610 * t8196;
    let t29465 = t6028 * t7257;
    let t29466 = t7948 * t29465;
    let t29468 = t28589 * t8191;
    let t29470 = t5909 * t6922;
    (t29459, t29461, t29463, t29465, t29466, t29468, t29470)
}
