//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 1027/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk1027(t1066: f64, t7062: f64, t268: f64, t8449: f64, t103: f64, t332: f64, t875: f64, t9739: f64, t147: f64, t19: f64, t2315: f64, t3295: f64, t966: f64) -> (f64, f64, f64, f64) {
    let t24007 = t1066 * t7062;
    let t24081 = t8449 * t268;
    let t24086 = t9739 * t332 * t103 * t875;
    let t24092 = t3295 * t966 * t2315 * t19 * t147;
    (t24007, t24081, t24086, t24092)
}
