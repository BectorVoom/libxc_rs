//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 1048/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk1048(t19: f64, t4864: f64, t1908: f64, t3940: f64, t11207: f64, t8351: f64, t147: f64, t1509: f64, t3155: f64, t681: f64, t2920: f64, t423: f64) -> (f64, f64, f64, f64, f64) {
    let t25076 = t4864 * t19;
    let t25110 = t3940 * t1908;
    let t25117 = t8351 * t11207;
    let t25127 = t3155 * t681 * t1509 * t19 * t147;
    let t25176 = t2920 * t423;
    (t25076, t25110, t25117, t25127, t25176)
}
