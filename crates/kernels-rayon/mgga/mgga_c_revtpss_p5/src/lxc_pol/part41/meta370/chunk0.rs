//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1206/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1206(t5941: f64, t72: f64, t757: f64, t10569: f64, t4186: f64, t4402: f64, t4401: f64, t177: f64, t762: f64, t10579: f64, t14386: f64, t1522: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18555 = t5941 * t72;
    let t18556 = t18555 * t757;
    let t18557 = 0.18311447306006545054e-3_f64 * t18556;
    let t18558 = 0.24415263074675393405e-3_f64 * t10569;
    let t18559 = t4402 * t4186;
    let t18561 = 24.0_f64 * t4401 * t18559;
    let t18562 = t5941 * t177;
    let t18563 = t18562 * t762;
    let t18564 = 0.5848223622634646207e0_f64 * t18563;
    let t18565 = 0.10843581300301739842e-1_f64 * t10579;
    let t18567 = 8.0_f64 * t14386 * t1522;
    (t18557, t18558, t18561, t18564, t18565, t18567)
}
