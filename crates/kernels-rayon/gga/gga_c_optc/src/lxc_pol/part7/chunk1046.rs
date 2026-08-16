//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1046/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1046(t138: f64, t6843: f64, t2053: f64, t658: f64, t2086: f64, t637: f64, t120: f64, t6916: f64, t1928: f64, t121: f64, t123: f64, t124: f64, t1948: f64, t2057: f64, t2060: f64, t2061: f64, t2064: f64, t21868: f64, t22052: f64, t22268: f64, t22269: f64, t22286: f64, t22628: f64, t22653: f64, t22678: f64, t22698: f64, t22729: f64, t3411: f64, t641: f64, t642: f64, t6560: f64, t6847: f64, t6857: f64, t6860: f64, t6861: f64, t6864: f64, t9742: f64, t9747: f64) -> (f64, f64) {
    let t22736 = t6843 * t138;
    let t22739 = t2053 * t658;
    let t22744 = t637 * t2086;
    let t22751 = t120 * t6916;
    let t22752 = t1928 * t1928;
    let t22769 = -0.12897460341341234505e3_f64 * (t22268 + t22269 + t22286 + t22628 + t22653 + t22678 + t22698 + t22729) * t121 * t124 + 0.15476952409609481406e4_f64 * t22736 * t642 - 0.92861714457656888434e4_f64 * t22739 * t2061 + 0.23215428614414222108e4_f64 * t6847 * t2064 + 0.30953904819218962812e5_f64 * t22744 * t6857 - 0.18572342891531377687e5_f64 * t9742 * t6861 + 0.15476952409609481406e4_f64 * t2057 * t6864 - 0.46430857228828444218e5_f64 * t22751 * t124 * t22752 + 0.46430857228828444218e5_f64 * t9747 * t123 * t1928 * t1948 - 0.46430857228828444218e4_f64 * t2060 * t124 * t21868 - 0.61907809638437925624e4_f64 * t3411 * t6860 * t6560 + 0.38692381024023703515e3_f64 * t641 * t124 * t22052;
    (t22752, t22769)
}
