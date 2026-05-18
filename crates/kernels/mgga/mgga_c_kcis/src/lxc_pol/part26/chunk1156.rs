//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1156/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1156<F: Float>(t17311: F, t8186: F, t5897: F, t8207: F, t2253: F, t7271: F, t12345: F, t2069: F, t4189: F, t7397: F, t6028: F, t6927: F) -> (F, F, F, F, F, F, F, F, F) {
    let t29421 = F::new(4.0) * t17311 * t8186;
    let t29423 = F::new(2.0) * t5897 * t8207;
    let t29424 = t2253 * t7271;
    let t29426 = F::new(6.0) * t12345 * t29424;
    let t29427 = t8207 * t2069;
    let t29429 = F::new(4.0) * t4189 * t29427;
    let t29430 = t2253 * t7397;
    let t29432 = F::new(2.0) * t4189 * t29430;
    let t29433 = t6028 * t6927;
    (t29421, t29423, t29424, t29426, t29427, t29429, t29430, t29432, t29433)
}
