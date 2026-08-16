//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1007/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1007(t1571: f64, t7460: f64, t4358: f64, t7459: f64, t6097: f64, t6101: f64, t12732: f64, t7443: f64, t1354: f64, t7469: f64, t2084: f64, t5613: f64) -> (f64, f64, f64, f64, f64, f64) {
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
