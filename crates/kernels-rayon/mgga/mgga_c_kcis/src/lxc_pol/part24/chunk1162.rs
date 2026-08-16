//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1162/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1162(t7579: f64, t9232: f64, t7592: f64, t7583: f64, t36962: f64, t26571: f64, t26602: f64, t26615: f64, t26597: f64, t26576: f64, t26607: f64, t26611: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t92270 = t9232 * t7579;
    let t92271 = t92270 * t7592;
    let t92273 = t92270 * t7583;
    let t92276 = t36962 * t7579 * t7583;
    let t92278 = t26602 * t26571;
    let t92280 = t26602 * t26615;
    let t92282 = t26597 * t26571;
    let t92284 = t26607 * t26576;
    let t92286 = t26607 * t26611;
    (t92271, t92273, t92276, t92278, t92280, t92282, t92284, t92286)
}
