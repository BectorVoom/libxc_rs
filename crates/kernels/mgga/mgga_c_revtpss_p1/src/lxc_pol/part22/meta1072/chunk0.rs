//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3842/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3842<F: Float>(t4011: F, t61999: F, t246: F, t5674: F, t13783: F, t13784: F, t13789: F, t13790: F, t13791: F, t1399: F, t21990: F, t22046: F, t22279: F, t22841: F, t3934: F, t4004: F, t46596: F, t46924: F, t48073: F, t48111: F, t48466: F, t49107: F, t5671: F, t5673: F, t5675: F, t5745: F, t73847: F, t9955: F) -> (F, F, F) {
    let t73906 = t61999 * t4011;
    let t73908 = t246 * t5674;
    let t73914 = F::cast_from(0.85748036236139473944e-3_f64) * t5671 * t5673 * t73847 * t5675 + F::cast_from(0.85748036236139473945e-2_f64) * t5671 * t9955 * t22046 * t46924 + F::cast_from(0.30011812682648815881e-2_f64) * t5671 * t5673 * t22046 * t4004 + F::cast_from(0.32012600194825403606e-1_f64) * t48111 - F::cast_from(0.17149607247227894789e-1_f64) * t3934 * t13783 * t22279 * t1399 - F::cast_from(0.68598428988911579156e-2_f64) * t5671 * t13789 * t21990 * t13784 - F::cast_from(0.68598428988911579156e-2_f64) * t5671 * t13789 * t13790 * t49107 - F::cast_from(0.34299214494455789578e-2_f64) * t5671 * t13789 * t13790 * t48466 - F::cast_from(0.10289764348336736873e-1_f64) * t5671 * t13789 * t48073 * t22841 + F::cast_from(0.34299214494455789578e-1_f64) * t5745 * t73906 * t73908 * t22841 * t13791 + F::cast_from(0.30234122406223992295e0_f64) * t46596;
    (t73906, t73908, t73914)
}
