//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1165/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1165<F: Float>(t33: F, t265: F, t502: F, t114149: F, t114199: F, t114089: F, t1469: F, t2003: F, t22671: F, t29978: F, t57: F, t5825: F, t7877: F, t2014: F, t30111: F, t5542: F, t101473: F, t29498: F, t113063: F, t113065: F, t113067: F, t113076: F, t113078: F, t113084: F, t113086: F, t113089: F, t113092: F, t113095: F, t114100: F, t118: F, t18245: F, t1911: F, t1932: F, t2007: F, t22634: F, t22747: F, t25043: F, t30150: F, t5877: F, t5884: F, t6985: F, t7746: F, t7883: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F,) {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t114200 = t114149 + t114199;
    let t114201 = piecewise3(t503, 0.0, t114089);
    let t114211 = piecewise3(t400, t114200, t114201 * t57 / 2.0 - 3.0 / 2.0 * t29978 * t1469 - 3.0 / 2.0 * t7877 * t5825 - t2003 * t22671 / 2.0);
    let t114216 = 3.0 * t2014 * t30111 * t5542;
    let t114221 = 18.0 * t2014 * t101473 * t29498;
    let t114222 = -t113063 - t113065 - t113067 - 6.0 * t5884 * t7883 - t22747 * t2007 - 3.0 * t5877 * t7883 - t1932 * t25043 + t113076 - t113078 - 2.0 * t6985 * t22634 - 6.0 * t18245 * t7746 - t113084 - t113086 - t113089 + t113092 + t113095 - t118 * (t114100 + t114211) - t114216 + 3.0 * t30150 * t1911 + t114221;
    (t114222,)
}
