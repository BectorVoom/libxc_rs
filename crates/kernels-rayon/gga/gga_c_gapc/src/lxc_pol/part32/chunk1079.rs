//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 1079/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk1079(t18317: f64, t33148: f64, t7259: f64, t11974: f64, t3285: f64, t3289: f64, t2572: f64, t33328: f64, t11397: f64, t932: f64, t11417: f64, t128: f64, t7333: f64, t935: f64) -> (f64, f64, f64, f64, f64) {
    let t33356 = t7259 * t33148 * t18317;
    let t33358 = t11974 * t3285;
    let t33360 = t11974 * t3289;
    let t33363 = t33328 * t2572;
    let t33364 = t932 * t11397 * t33363;
    let t33369 = t932 * t11417 * t7333 * t935 * t128;
    (t33356, t33358, t33360, t33364, t33369)
}
