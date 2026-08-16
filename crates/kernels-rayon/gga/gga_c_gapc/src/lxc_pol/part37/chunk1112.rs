//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1112/1445 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1112(t11207: f64, t8351: f64, t147: f64, t1509: f64, t19: f64, t3155: f64, t681: f64, t2920: f64, t423: f64, t1338: f64, t3156: f64, t1403: f64, t3116: f64) -> (f64, f64, f64, f64, f64) {
    let t25117 = t8351 * t11207;
    let t25127 = t3155 * t681 * t1509 * t19 * t147;
    let t25176 = t2920 * t423;
    let t25202 = t3156 * t1338 * t19 * t147;
    let t25382 = t3116 * t1403 * t19 * t147;
    (t25117, t25127, t25176, t25202, t25382)
}
