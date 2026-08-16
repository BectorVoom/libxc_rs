//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 937/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk937(t225: f64, t4149: f64, t4351: f64, t892: f64, t1543: f64, t2841: f64, t4389: f64, t699: f64, t4386: f64, t4339: f64, t690: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13463 = t4149 * t225;
    let t13515 = t4351 * t892;
    let t13520 = t1543 * t2841;
    let t13550 = t699 * t4389;
    let t13551 = 0.21908444444444444444e0_f64 * t13550;
    let t13552 = t699 * t4386;
    let t13563 = t690 * t4339;
    (t13463, t13515, t13520, t13550, t13551, t13552, t13563)
}
