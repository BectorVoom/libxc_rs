//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1117/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1117(t7259: f64, t9709: f64, t1389: f64, t3964: f64, t92986: f64, t26009: f64, t9802: f64, t64: f64, t9990: f64, t239: f64, t820: f64, t2482: f64, t596: f64, t7262: f64) -> (f64, f64, f64, f64, f64) {
    let t94473 = t7259 * t9709;
    let t94476 = t3964 * t92986 * t1389;
    let t94483 = t9802 * t26009;
    let t94491 = t9990 * t64;
    let t94493 = t820 * t94491 * t239;
    let t94497 = t2482 * t7262 * t596;
    (t94473, t94476, t94483, t94493, t94497)
}
