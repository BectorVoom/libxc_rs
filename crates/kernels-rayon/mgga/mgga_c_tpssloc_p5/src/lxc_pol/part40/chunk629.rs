//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 629/1303 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk629(t1354: f64, t3799: f64, t1339: f64, t242: f64, t1336: f64, t1365: f64, t67: f64, t246: f64) -> (f64, f64, f64, f64, f64) {
    let t3800 = t3799 * t1354;
    let t3802 = t1339 * t242;
    let t3803 = t1336 * t3802;
    let t3804 = t1365 * t67;
    let t3805 = t3804 * t246;
    (t3800, t3802, t3803, t3804, t3805)
}
