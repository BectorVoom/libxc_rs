//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2061/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2061(t1955: f64, t43603: f64, t4657: f64, t6688: f64, t7566: f64, t82632: f64, t23384: f64, t25400: f64, t25416: f64, t82431: f64, t1921: f64, t88804: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t88851 = t43603 * t1955;
    let t88868 = t6688 * t4657;
    let t88882 = t82632 * t7566;
    let t88889 = 0.54831135561607547884e-2_f64 * t23384 * t25400;
    let t88915 = 0.18277045187202515961e-2_f64 * t82431 * t25416;
    let t88932 = t1921 * t88804;
    (t88851, t88868, t88882, t88889, t88915, t88932)
}
