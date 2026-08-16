//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 523/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk523(t236: f64, t3787: f64, t550: f64, t1339: f64, t835: f64, t1336: f64, t1354: f64, t242: f64, t1365: f64, t67: f64, t246: f64, t120: f64, t1351: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3788 = t3787 * t236;
    let t3792 = t550 * t550;
    let t3798 = t1339 * t835;
    let t3799 = t1336 * t3798;
    let t3800 = t3799 * t1354;
    let t3802 = t1339 * t242;
    let t3803 = t1336 * t3802;
    let t3804 = t1365 * t67;
    let t3805 = t3804 * t246;
    let t3806 = t120 * t1351;
    (t3788, t3792, t3799, t3800, t3803, t3805, t3806)
}
