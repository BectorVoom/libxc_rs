//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 997/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk997<F: Float>(t126101: F, t126153: F, t126205: F, t126241: F, t126290: F, t126333: F, t126367: F, t126408: F, t892: F, t198: F, t205: F, t8489: F, t119737: F, t119747: F, t126006: F, t126007: F, t126013: F, t126014: F, t126018: F, t126027: F, t126031: F, t126037: F, t1468: F, t1940: F, t2403: F, t25206: F, t25440: F, t27160: F, t27166: F, t27169: F, t27173: F, t27382: F, t27387: F, t27395: F, t27402: F, t30: F, t31859: F, t31863: F, t31876: F, t33727: F, t33740: F, t7010: F, t7091: F, t7787: F, t8490: F, t8494: F) -> (F, F, F, F) {
    let t126411 = t126101 + t126153 + t126205 + t126241 + t126290 + t126333 + t126367 + t126408;
    let t126412 = t126411 * t892;
    let t126422 = t198 * t205 * t8489;
    let t126433 = t126006 - t1940 * t7091 * t126007 - t1940 * t25440 * t33740 - 3.0 * t126013 * t126014 + 2.0 * t27382 * t126018 + t1940 * t31859 * t1468 / 2.0 - t1940 * t119737 * t7787 / 2.0 - t1940 * t7091 * t126027 - 3.0 * t25206 * t126031 + 3.0 / 2.0 * t2403 * t8490 * t27173 - t1940 * t7091 * t126037 + 3.0 / 2.0 * t2403 * t8490 * t27169 + t1940 * t126412 * t30 / 2.0 - 3.0 / 2.0 * t2403 * t8494 * t27395 - 3.0 / 2.0 * t119747 * t27166 + 3.0 * t126422 * t27160 - t1940 * t31863 * t27402 / 2.0 + t1940 * t31876 * t27387 + 3.0 / 2.0 * t2403 * t33727 * t7010;
    (t126411, t126412, t126422, t126433)
}
