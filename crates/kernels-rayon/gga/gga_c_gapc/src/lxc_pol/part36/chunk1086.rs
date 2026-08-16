//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 1086/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk1086(t11902: f64, t19161: f64, t18317: f64, t33148: f64, t7259: f64, t11974: f64, t3285: f64, t3289: f64, t2572: f64, t33328: f64, t11397: f64, t932: f64) -> (f64, f64, f64, f64, f64) {
    let t33353 = t11902 * t19161;
    let t33356 = t7259 * t33148 * t18317;
    let t33358 = t11974 * t3285;
    let t33360 = t11974 * t3289;
    let t33363 = t33328 * t2572;
    let t33364 = t932 * t11397 * t33363;
    (t33353, t33356, t33358, t33360, t33364)
}
