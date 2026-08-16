//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1099/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1099(t3431: f64, t418: f64, t408: f64, t3418: f64, t698: f64, t240: f64, t3698: f64, t3361: f64, t635: f64, t1146: f64, t2439: f64, t3424: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12247 = 1.0_f64 / t3431 / t418;
    let t12248 = t408 * t12247;
    let t12252 = t698 * t3418;
    let t12254 = t240 * t3698;
    let t12256 = 1.0_f64 / t3361 / t635;
    let t12261 = t2439 * t1146;
    let t12263 = t698 * t3424;
    (t12248, t12252, t12254, t12256, t12261, t12263)
}
