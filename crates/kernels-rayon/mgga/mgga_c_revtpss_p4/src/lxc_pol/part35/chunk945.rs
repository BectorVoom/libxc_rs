//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 945/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk945(t15123: f64, t15189: f64, t23472: f64, t23476: f64, t23479: f64, t23483: f64, t23487: f64, t23490: f64, t23493: f64, t23496: f64, t23501: f64, t23505: f64, t23508: f64, t23511: f64) -> f64 {
    let t23680 = -0.36793333333333333333e-1_f64 * t23472 - 0.82785e-1_f64 * t23476 - 0.33547222222222222222e0_f64 * t23479 + 0.12077e1_f64 * t23483 - 0.181155e1_f64 * t23487 - 0.301925e0_f64 * t23490 + 0.16557e0_f64 * t23493 - 0.49671e0_f64 * t23496 - 0.27595e0_f64 * t15123 - 0.60384999999999999999e0_f64 * t23501 + 0.181155e1_f64 * t23505 - 0.82785e-1_f64 * t23508 + 0.49671e0_f64 * t23511 - 0.40256666666666666668e0_f64 * t15189;
    t23680
}
