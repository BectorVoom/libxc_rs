//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1322/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1322(t114149: f64, t114199: f64, t2014: f64, t30111: f64, t5542: f64, t101473: f64, t29498: f64, t29502: f64, t4248: f64, t22483: f64, t7934: f64, t1497: f64, t29547: f64, t77: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t114200 = t114149 + t114199;
    let t114216 = 3.0_f64 * t2014 * t30111 * t5542;
    let t114221 = 18.0_f64 * t2014 * t101473 * t29498;
    let t114230 = 12.0_f64 * t4248 * t29502;
    let t114238 = 3.0_f64 * t2014 * t7934 * t22483;
    let t114246 = t77 * t29547 * t1497;
    (t114200, t114216, t114221, t114230, t114238, t114246)
}
