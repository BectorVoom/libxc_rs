//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1185/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1185(t1711: f64, t5966: f64, t6079: f64, t23279: f64, t27763: f64, t6075: f64, t23421: f64, t33: f64, t113096: f64, t25759: f64, t23148: f64, t1583: f64, t6416: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t114117 = t1711 * t5966;
    let t114121 = t1711 * t6079;
    let t114128 = t27763 * t23279;
    let t114140 = t1711 * t6075;
    let t114150 = t33 * t23421;
    let t114165 = t25759 * t113096;
    let t114171 = t33 * t23148;
    let t114184 = t6416 * t1583;
    (t114117, t114121, t114128, t114140, t114150, t114165, t114171, t114184)
}
