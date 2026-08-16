//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2011/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2011(t533: f64, t816: f64, t92993: f64, t7259: f64, t9709: f64, t1389: f64, t3964: f64, t92986: f64, t7028: f64, t9736: f64, t9737: f64, t26009: f64, t9802: f64) -> (f64, f64, f64, f64, f64) {
    let t94471 = t92993 * t533 * t816;
    let t94472 = 455.0_f64 / 1296.0_f64 * t94471;
    let t94473 = t7259 * t9709;
    let t94474 = 0.25692334753583138159e-2_f64 * t94473;
    let t94476 = t3964 * t92986 * t1389;
    let t94477 = 0.16264433699083676445e-3_f64 * t94476;
    let t94479 = t9736 * t7028 * t9737;
    let t94483 = t9802 * t26009;
    (t94472, t94474, t94477, t94479, t94483)
}
