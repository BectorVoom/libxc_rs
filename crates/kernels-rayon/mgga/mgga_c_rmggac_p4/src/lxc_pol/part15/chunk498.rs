//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 498/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk498(t1004: f64, t1425: f64, t1528: f64, t4290: f64, t4324: f64, t4328: f64, t436: f64, t4361: f64, t4365: f64, t5372: f64, t5464: f64, t5471: f64, t6004: f64, t6006: f64, t6007: f64, t6008: f64, t6009: f64, t6010: f64, t6011: f64, t6012: f64, t6013: f64, t6014: f64, t6067: f64, t619: f64) -> f64 {
    let t6301 = t6004 + t4290 - t6006 + t6007 - 0.62182e-1_f64 * t619 * t1004 * t1528 + t4361 - t4365 + t6008 + t6009 - t6010 + t4324 - t6011 + t4328 + 0.93273e-1_f64 * t436 * t6067 - t5464 + t6012 - t6013 + 0.186546e0_f64 * t1425 * t5372 + t5471 - t6014;
    t6301
}
