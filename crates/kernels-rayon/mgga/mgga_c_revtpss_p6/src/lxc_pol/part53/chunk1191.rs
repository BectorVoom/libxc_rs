//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1191/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1191(t119737: f64, t119747: f64, t126006: f64, t126007: f64, t126013: f64, t126014: f64, t126018: f64, t126027: f64, t126031: f64, t126037: f64, t126412: f64, t126422: f64, t1468: f64, t1940: f64, t2403: f64, t25206: f64, t25440: f64, t27160: f64, t27166: f64, t27169: f64, t27173: f64, t27382: f64, t27387: f64, t27395: f64, t27402: f64, t30: f64, t31859: f64, t31863: f64, t31876: f64, t33727: f64, t33740: f64, t7010: f64, t7091: f64, t7787: f64, t8490: f64, t8494: f64) -> f64 {
    let t126433 = t126006 - t1940 * t7091 * t126007 - t1940 * t25440 * t33740 - 3.0_f64 * t126013 * t126014 + 2.0_f64 * t27382 * t126018 + t1940 * t31859 * t1468 / 2.0_f64 - t1940 * t119737 * t7787 / 2.0_f64 - t1940 * t7091 * t126027 - 3.0_f64 * t25206 * t126031 + 3.0_f64 / 2.0_f64 * t2403 * t8490 * t27173 - t1940 * t7091 * t126037 + 3.0_f64 / 2.0_f64 * t2403 * t8490 * t27169 + t1940 * t126412 * t30 / 2.0_f64 - 3.0_f64 / 2.0_f64 * t2403 * t8494 * t27395 - 3.0_f64 / 2.0_f64 * t119747 * t27166 + 3.0_f64 * t126422 * t27160 - t1940 * t31863 * t27402 / 2.0_f64 + t1940 * t31876 * t27387 + 3.0_f64 / 2.0_f64 * t2403 * t33727 * t7010;
    t126433
}
