//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1347/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1347<F: Float>(t1395: F, t22456: F, t22406: F, t7948: F, t27514: F, t29465: F, t8196: F, t97772: F, t22630: F, t573: F, t28589: F, t28597: F) -> (F, F, F, F, F, F) {
    let t103022 = t1395 * t22456;
    let t103024 = t7948 * t22406;
    let t103026 = t27514 * t29465;
    let t103028 = t97772 * t8196;
    let t103031 = t22630 * t573;
    let t103033 = t28589 * t28597;
    (t103022, t103024, t103026, t103028, t103031, t103033)
}
