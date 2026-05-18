//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1155/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1155<F: Float>(t29296: F, t29335: F, t29377: F, t29410: F, t589: F, t2069: F, t28558: F, t27494: F, t7271: F, t7397: F, t7940: F, t22300: F, t2253: F) -> (F, F, F, F, F, F) {
    let t29412 = t29296 + t29335 + t29377 + t29410;
    let t29413 = t29412 * t589;
    let t29415 = F::new(2.0) * t28558 * t2069;
    let t29417 = F::new(2.0) * t27494 * t7271;
    let t29418 = t7940 * t7397;
    let t29419 = t22300 * t2253;
    (t29412, t29413, t29415, t29417, t29418, t29419)
}
