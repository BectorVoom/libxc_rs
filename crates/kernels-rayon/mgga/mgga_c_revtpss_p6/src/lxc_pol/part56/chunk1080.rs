//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1080/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1080(t12915: f64, t247: f64, t33431: f64, t8926: f64, t33501: f64, t97312: f64, t33502: f64, t3678: f64, t33455: f64, t7642: f64, t1209: f64, t8938: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t124964 = t8926 * t247 * t12915 * t33431;
    let t124984 = t33501 * t97312;
    let t124994 = t33502 * t3678;
    let t124996 = t7642 * t33455;
    let t125003 = t1209 * t33455;
    let t125009 = t8938 * t97312;
    (t124964, t124984, t124994, t124996, t125003, t125009)
}
