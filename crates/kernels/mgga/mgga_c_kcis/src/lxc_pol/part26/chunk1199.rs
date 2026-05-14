//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1199/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1199<F: Float>(t27517: F, t29479: F, t22387: F, t28624: F, t8196: F, t97784: F, t5913: F, t97801: F, t585: F, t59975: F, t1395: F, t22456: F, t22406: F, t7948: F, t27514: F, t29465: F) -> (F, F, F, F, F, F, F, F) {
    let t103012 = t27517 * t29479;
    let t103014 = t28624 * t22387;
    let t103016 = t97784 * t8196;
    let t103018 = t97801 * t5913;
    let t103020 = t59975 * t585;
    let t103022 = t1395 * t22456;
    let t103024 = t7948 * t22406;
    let t103026 = t27514 * t29465;
    (t103012, t103014, t103016, t103018, t103020, t103022, t103024, t103026)
}
