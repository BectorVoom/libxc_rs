//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 401/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk401(t1339: f64, t835: f64, t1336: f64, t242: f64, t1365: f64, t67: f64, t246: f64, t1291: f64, t2663: f64, t2225: f64, t522: f64, t2221: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3798 = t1339 * t835;
    let t3799 = t1336 * t3798;
    let t3802 = t1339 * t242;
    let t3803 = t1336 * t3802;
    let t3804 = t1365 * t67;
    let t3805 = t3804 * t246;
    let t3813 = 0.24415263074675393405e-3_f64 * t1291 * t2663;
    let t3819 = 20.0_f64 * t2225 * t522;
    let t3821 = 12.0_f64 * t2221 * t522;
    (t3799, t3803, t3805, t3813, t3819, t3821)
}
