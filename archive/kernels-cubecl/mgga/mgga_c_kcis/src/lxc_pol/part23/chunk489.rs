//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 489/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk489<F: Float>(t1452: F, t743: F, t1451: F, t3805: F, t1430: F, t3797: F, t1431: F, t733: F, t542: F, t1438: F, t738: F, t113: F, t3754: F) -> (F, F, F, F, F, F, F, F) {
    let t4073 = t743 * t1452;
    let t4075 = t1451 * t3805;
    let t4078 = t1430 * t3797;
    let t4081 = t733 * t1431;
    let t4083 = t1430 * t3805;
    let t4086 = t542 * t3797;
    let t4089 = t738 * t1438;
    let t4093 = t113 * t3754;
    (t4073, t4075, t4078, t4081, t4083, t4086, t4089, t4093)
}
