//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1307/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1307(t101473: f64, t2014: f64, t29498: f64, t113063: f64, t113065: f64, t113067: f64, t113076: f64, t113078: f64, t113084: f64, t113086: f64, t113089: f64, t113092: f64, t113095: f64, t114100: f64, t114211: f64, t114216: f64, t118: f64, t18245: f64, t1911: f64, t1932: f64, t2007: f64, t22634: f64, t22747: f64, t25043: f64, t30150: f64, t5877: f64, t5884: f64, t6985: f64, t7746: f64, t7883: f64) -> f64 {
    let t114221 = 18.0_f64 * t2014 * t101473 * t29498;
    let t114222 = -t113063 - t113065 - t113067 - 6.0_f64 * t5884 * t7883 - t22747 * t2007 - 3.0_f64 * t5877 * t7883 - t1932 * t25043 + t113076 - t113078 - 2.0_f64 * t6985 * t22634 - 6.0_f64 * t18245 * t7746 - t113084 - t113086 - t113089 + t113092 + t113095 - t118 * (t114100 + t114211) - t114216 + 3.0_f64 * t30150 * t1911 + t114221;
    t114222
}
