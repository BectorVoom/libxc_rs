//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2137/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2137(t1468: f64, t4433: f64, t892: f64, t1583: f64, t4537: f64, t27383: f64, t6079: f64, t775: f64, t890: f64, t98785: f64, t25207: f64, t77408: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t106546 = t892 * t1468 * t4433;
    let t106554 = t1583 * t4537;
    let t106555 = t27383 * t106554;
    let t106561 = t6079 * t775;
    let t106562 = t27383 * t106561;
    let t106565 = t6079 * t890;
    let t106566 = t98785 * t106565;
    let t106569 = t25207 * t77408;
    (t106546, t106554, t106555, t106561, t106562, t106565, t106566, t106569)
}
