//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1361/1427 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1361(t35662: f64, t35664: f64, t35638: f64, t36421: f64, t36422: f64, t36423: f64, t36425: f64, t36426: f64, t36427: f64, t36428: f64, t36429: f64, t36430: f64, t36431: f64) -> f64 {
    let t36432 = 0.809822844183586641e-4_f64 * t35662;
    let t36433 = 0.28073858598364336888e-2_f64 * t35664;
    let t36434 = t36421 + t36422 + t36423 + 0.54311401758461002391e-5_f64 * t35638 + t36425 + t36426 - t36427 - t36428 + t36429 + t36430 - t36431 + t36432 + t36433;
    t36434
}
