//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3842/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3842(t4011: f64, t61999: f64, t246: f64, t5674: f64, t13783: f64, t13784: f64, t13789: f64, t13790: f64, t13791: f64, t1399: f64, t21990: f64, t22046: f64, t22279: f64, t22841: f64, t3934: f64, t4004: f64, t46596: f64, t46924: f64, t48073: f64, t48111: f64, t48466: f64, t49107: f64, t5671: f64, t5673: f64, t5675: f64, t5745: f64, t73847: f64, t9955: f64) -> (f64, f64, f64) {
    let t73906 = t61999 * t4011;
    let t73908 = t246 * t5674;
    let t73914 = 0.85748036236139473944e-3_f64 * t5671 * t5673 * t73847 * t5675 + 0.85748036236139473945e-2_f64 * t5671 * t9955 * t22046 * t46924 + 0.30011812682648815881e-2_f64 * t5671 * t5673 * t22046 * t4004 + 0.32012600194825403606e-1_f64 * t48111 - 0.17149607247227894789e-1_f64 * t3934 * t13783 * t22279 * t1399 - 0.68598428988911579156e-2_f64 * t5671 * t13789 * t21990 * t13784 - 0.68598428988911579156e-2_f64 * t5671 * t13789 * t13790 * t49107 - 0.34299214494455789578e-2_f64 * t5671 * t13789 * t13790 * t48466 - 0.10289764348336736873e-1_f64 * t5671 * t13789 * t48073 * t22841 + 0.34299214494455789578e-1_f64 * t5745 * t73906 * t73908 * t22841 * t13791 + 0.30234122406223992295e0_f64 * t46596;
    (t73906, t73908, t73914)
}
