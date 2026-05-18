//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1387/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1387<F: Float>(t1571: F, t7460: F, t4358: F, t7459: F, t6097: F, t6101: F, t12732: F, t7443: F, t1354: F, t7469: F, t2084: F, t5613: F) -> (F, F, F, F, F, F) {
    let t22843 = t7460 * t1571;
    let t22846 = t7459 * t4358;
    let t22847 = t22846 * t1571;
    let t22850 = t6101 * t6097;
    let t22853 = t7443 * t12732;
    let t22854 = t22853 * t1571;
    let t22861 = t7469 * t1354;
    let t22864 = t2084 * t5613;
    (t22843, t22847, t22850, t22854, t22861, t22864)
}
