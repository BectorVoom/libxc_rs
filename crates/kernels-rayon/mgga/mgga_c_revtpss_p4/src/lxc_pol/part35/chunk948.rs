//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 948/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk948(t23705: f64, t2970: f64, t15123: f64, t15189: f64, t23472: f64, t23476: f64, t23479: f64, t23483: f64, t23487: f64, t23490: f64, t23493: f64, t23496: f64, t23501: f64, t23505: f64, t23508: f64, t23511: f64) -> (f64, f64) {
    let t23723 = t23705 * t2970;
    let t23740 = -0.46308888888888888889e-1_f64 * t23472 - 0.104195e0_f64 * t23476 - 0.57386111111111111112e0_f64 * t23479 + 0.20659e1_f64 * t23483 - 0.309885e1_f64 * t23487 - 0.516475e0_f64 * t23490 + 0.20839e0_f64 * t23493 - 0.62517e0_f64 * t23496 - 0.34731666666666666667e0_f64 * t15123 - 0.103295e1_f64 * t23501 + 0.309885e1_f64 * t23505 - 0.104195e0_f64 * t23508 + 0.62517e0_f64 * t23511 - 0.68863333333333333332e0_f64 * t15189;
    (t23723, t23740)
}
