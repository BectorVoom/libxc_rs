//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 523/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk523(t1339: f64, t835: f64, t1336: f64, t242: f64, t1365: f64, t67: f64, t246: f64) -> (f64, f64, f64, f64, f64) {
    let t3798 = t1339 * t835;
    let t3799 = t1336 * t3798;
    let t3802 = t1339 * t242;
    let t3803 = t1336 * t3802;
    let t3804 = t1365 * t67;
    let t3805 = t3804 * t246;
    (t3798, t3799, t3802, t3803, t3805)
}
