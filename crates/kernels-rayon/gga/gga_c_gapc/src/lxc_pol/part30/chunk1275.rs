//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 1275/1331 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk1275(t1056: f64, t3217: f64, t7371: f64, t2207: f64, t22672: f64, t2580: f64, t474: f64, t11613: f64, t2786: f64, t996: f64, t11616: f64, t3212: f64) -> (f64, f64, f64, f64) {
    let t35999 = t3217 * t1056 * t7371;
    let t36003 = t22672 * t2207 * t474 * t2580;
    let t36006 = t996 * t2786 * t11613;
    let t36009 = t3212 * t11616;
    (t35999, t36003, t36006, t36009)
}
