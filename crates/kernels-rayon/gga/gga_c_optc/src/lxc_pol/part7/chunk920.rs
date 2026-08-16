//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 920/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk920(t8639: f64, t8642: f64, t8628: f64, t8630: f64, t8632: f64, t8636: f64, t8645: f64, t8648: f64, t8651: f64, t8654: f64, t8657: f64, t8660: f64, t8674: f64, t8676: f64) -> f64 {
    let t8727 = 0.93932222222222222223e0_f64 * t8639;
    let t8728 = 0.36793333333333333333e0_f64 * t8642;
    let t8737 = -0.27595e0_f64 * t8628 + 0.16557e0_f64 * t8630 + 0.5519e-1_f64 * t8632 - 0.36793333333333333333e-1_f64 * t8636 - t8727 - t8728 - 0.3883875e1_f64 * t8645 + 0.247573125e0_f64 * t8648 - 0.82785e-1_f64 * t8651 + 0.49671e0_f64 * t8654 - 0.60384999999999999999e0_f64 * t8657 + 0.181155e1_f64 * t8660 + 0.16504875e0_f64 * t8674 + 0.258925e1_f64 * t8676;
    t8737
}
