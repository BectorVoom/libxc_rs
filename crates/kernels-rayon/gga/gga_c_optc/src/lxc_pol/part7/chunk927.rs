//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 927/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk927(t8639: f64, t8642: f64, t8628: f64, t8630: f64, t8632: f64, t8636: f64, t8645: f64, t8648: f64, t8651: f64, t8654: f64, t8657: f64, t8660: f64, t8674: f64, t8676: f64) -> f64 {
    let t8831 = 0.16068111111111111111e1_f64 * t8639;
    let t8832 = 0.46308888888888888888e0_f64 * t8642;
    let t8841 = -0.34731666666666666667e0_f64 * t8628 + 0.20839e0_f64 * t8630 + 0.69463333333333333335e-1_f64 * t8632 - 0.46308888888888888889e-1_f64 * t8636 - t8831 - t8832 - 0.52945875e1_f64 * t8645 + 0.94674375e0_f64 * t8648 - 0.104195e0_f64 * t8651 + 0.62517e0_f64 * t8654 - 0.103295e1_f64 * t8657 + 0.309885e1_f64 * t8660 + 0.6311625e0_f64 * t8674 + 0.3529725e1_f64 * t8676;
    t8841
}
