//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3476/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3476(t15547: f64, t4725: f64, t1642: f64, t52921: f64, t4729: f64, t4734: f64, t64465: f64, t64467: f64, t64471: f64, t64475: f64, t64483: f64, t65402: f64, t65404: f64, t65408: f64, t65413: f64) -> (f64, f64, f64, f64, f64) {
    let t65415 = 0.46785788981077169656e1_f64 * t15547 * t4725;
    let t65417 = 0.11696447245269292414e1_f64 * t52921 * t1642;
    let t65419 = 0.23392894490538584828e1_f64 * t15547 * t4729;
    let t65421 = 0.69263436422725855034e2_f64 * t15547 * t4734;
    let t65422 = t64465 + t65402 + t64467 - t65404 + t65408 - t65413 + t65415 - t65417 - t65419 - t65421 + t64471 + t64475 + t64483;
    (t65415, t65417, t65419, t65421, t65422)
}
