//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 1085/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk1085(t11387: f64, t15650: f64, t7204: f64, t8785: f64, t8910: f64, t15610: f64, t2660: f64, t128: f64, t332: f64, t11969: f64, t3327: f64, t818: f64) -> (f64, f64, f64, f64, f64) {
    let t33427 = t7204 * t11387 * t15650;
    let t33429 = t8910 * t8785;
    let t33431 = t2660 * t33429 * t15610;
    let t33433 = t332 * t128;
    let t33436 = t11969 * t3327 * t33433 * t818;
    (t33427, t33429, t33431, t33433, t33436)
}
