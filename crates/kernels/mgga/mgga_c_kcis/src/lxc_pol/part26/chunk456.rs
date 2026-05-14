//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 456/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk456<F: Float>(t4061: F, t833: F, t1452: F, t743: F, t1431: F, t733: F, t1438: F, t738: F, t113: F, t3754: F, t3245: F, t558: F, t1014: F, t1460: F, t1465: F, t551: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4062 = t4061 * t833;
    let t4073 = t743 * t1452;
    let t4081 = t733 * t1431;
    let t4089 = t738 * t1438;
    let t4093 = t113 * t3754;
    let t4114 = t3245 * t558;
    let t4115 = 0.55273148148148148147e-3 * t4114;
    let t4117 = t1014 * t1460;
    let t4121 = 1.0 / t1465 / t551;
    (t4062, t4073, t4081, t4089, t4093, t4114, t4115, t4117, t4121)
}
