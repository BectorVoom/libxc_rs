//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1103/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1103(t22765: f64, t3858: f64, t22764: f64, t3777: f64, t1354: f64, t22756: f64, t1336: f64, t22759: f64, t835: f64, t3795: f64, t22760: f64, t3853: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t80989 = t22765 * t3858;
    let t80991 = t3777 * t22764;
    let t80992 = t80991 * t1354;
    let t80994 = t22756 * t3858;
    let t80997 = t1336 * t22759 * t835;
    let t80998 = t80997 * t3795;
    let t81000 = t3777 * t22760;
    let t81001 = t81000 * t3795;
    let t81003 = t22756 * t3853;
    (t80989, t80992, t80994, t80998, t81001, t81003)
}
