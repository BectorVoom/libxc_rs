//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 913/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk913(t231: f64, t23244: f64, t23168: f64, t827: f64, t828: f64, t23172: f64, t124: f64, t23114: f64, t800: f64, t23148: f64, t1544: f64, t5984: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t23245 = t23244 * t231;
    let t23253 = t827 * t828 * t23168;
    let t23257 = t827 * t828 * t23172;
    let t23262 = t124 * t23114;
    let t23263 = t800 * t23262;
    let t23266 = t124 * t23148;
    let t23267 = t800 * t23266;
    let t23275 = t800 * t5984 * t1544;
    (t23245, t23253, t23257, t23263, t23267, t23275)
}
