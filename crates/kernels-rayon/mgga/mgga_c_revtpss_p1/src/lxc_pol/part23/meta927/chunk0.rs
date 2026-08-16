//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3009/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3009(t1011: f64, t140: f64, t23868: f64, t41361: f64, t42078: f64, t51978: f64, t53243: f64, t63276: f64, t63278: f64, t77499: f64, t77503: f64, t77505: f64, t77507: f64, t77509: f64, t77515: f64, t77518: f64, t77521: f64, t77527: f64, t77531: f64, t77535: f64, t77539: f64, t77543: f64, t77547: f64) -> (f64, f64) {
    let t79957 = t1011 * t140 * t23868;
    let t80008 = 0.5487654320987654321e-2_f64 * t77499 - 0.14816666666666666667e-1_f64 * t77503 + 0.4938888888888888889e-2_f64 * t77505 - 0.19755555555555555556e-1_f64 * t77507 + 0.29633333333333333334e-1_f64 * t77509 - 0.29633333333333333334e-1_f64 * t63276 + 0.9877777777777777778e-2_f64 * t63278 + t42078 + 0.17780000000000000001e0_f64 * t77515 - 0.4938888888888888889e-1_f64 * t77518 - 0.26670000000000000001e0_f64 * t77521 - t53243 + 0.46096296296296296297e-1_f64 * t51978 + 0.15365432098765432099e-1_f64 * t41361 - 0.29633333333333333334e-1_f64 * t77527 - 0.29633333333333333334e-1_f64 * t77531 + 0.35560000000000000001e0_f64 * t77535 - 0.26670000000000000001e0_f64 * t77539 + 0.88900000000000000002e-1_f64 * t77543 + 0.88900000000000000002e-1_f64 * t77547;
    (t79957, t80008)
}
