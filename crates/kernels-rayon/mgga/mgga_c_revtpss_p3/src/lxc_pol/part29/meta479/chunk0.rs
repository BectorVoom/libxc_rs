//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1755/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1755(t4345: f64, t7045: f64, t25234: f64, t4349: f64, t25227: f64, t4353: f64, t2661: f64, t1565: f64, t25222: f64, t241: f64, t25260: f64, t820: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t27249 = t7045 * t4345;
    let t27251 = t25234 * t4349;
    let t27253 = t25227 * t4353;
    let t27254 = t2661 * t27253;
    let t27256 = t25222 * t1565;
    let t27261 = t820 * t25260 * t241;
    (t27249, t27251, t27253, t27254, t27256, t27261)
}
