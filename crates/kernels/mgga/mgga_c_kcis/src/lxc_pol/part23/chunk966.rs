//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 966/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk966<F: Float>(t1356: F, t16330: F, t2072: F, t4355: F, t1571: F, t6098: F, t2080: F, t4350: F, t4332: F, t6101: F, t4358: F, t6097: F) -> (F, F, F, F, F, F) {
    let t17792 = t16330 * t1356;
    let t17797 = t2072 * t4355;
    let t17806 = t6098 * t1571;
    let t17809 = t2080 * t4350;
    let t17812 = t6101 * t4332;
    let t17815 = t6097 * t4358;
    (t17792, t17797, t17806, t17809, t17812, t17815)
}
