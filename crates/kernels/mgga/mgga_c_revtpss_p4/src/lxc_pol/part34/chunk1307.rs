//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1307/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1307<F: Float>(t101473: F, t2014: F, t29498: F, t113063: F, t113065: F, t113067: F, t113076: F, t113078: F, t113084: F, t113086: F, t113089: F, t113092: F, t113095: F, t114100: F, t114211: F, t114216: F, t118: F, t18245: F, t1911: F, t1932: F, t2007: F, t22634: F, t22747: F, t25043: F, t30150: F, t5877: F, t5884: F, t6985: F, t7746: F, t7883: F) -> F {
    let t114221 = F::cast_from(18.0_f64) * t2014 * t101473 * t29498;
    let t114222 = -t113063 - t113065 - t113067 - F::cast_from(6.0_f64) * t5884 * t7883 - t22747 * t2007 - F::cast_from(3.0_f64) * t5877 * t7883 - t1932 * t25043 + t113076 - t113078 - F::cast_from(2.0_f64) * t6985 * t22634 - F::cast_from(6.0_f64) * t18245 * t7746 - t113084 - t113086 - t113089 + t113092 + t113095 - t118 * (t114100 + t114211) - t114216 + F::cast_from(3.0_f64) * t30150 * t1911 + t114221;
    t114222
}
