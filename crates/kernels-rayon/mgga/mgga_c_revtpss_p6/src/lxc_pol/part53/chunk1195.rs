//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1195/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1195(t125961: f64, t27799: f64, t27363: f64, t33: f64, t25759: f64, t4433: f64, t119706: f64, t119737: f64, t119747: f64, t125968: f64, t125976: f64, t125980: f64, t126013: f64, t126422: f64, t127190: f64, t127193: f64, t127199: f64, t127204: f64, t127207: f64, t127212: f64, t1940: f64, t2403: f64, t25206: f64, t25440: f64, t27382: f64, t27764: f64, t27770: f64, t27777: f64, t27800: f64, t31859: f64, t33727: f64, t33888: f64, t7091: f64, t7200: f64, t7862: f64, t7869: f64, t8494: f64) -> f64 {
    let t127218 = t27799 * t125961;
    let t127227 = t33 * t27363;
    let t127233 = t25759 * t4433;
    let t127236 = 3.0_f64 * t119706 * t127190 - 3.0_f64 * t25206 * t127193 + 3.0_f64 * t126422 * t27764 + t125968 * t27800 - 3.0_f64 * t25206 * t127199 - 3.0_f64 / 2.0_f64 * t119747 * t27770 - 3.0_f64 * t125980 * t127204 - t1940 * t7091 * t127207 - t1940 * t25440 * t33888 - t1940 * t7091 * t127212 + 3.0_f64 / 2.0_f64 * t2403 * t33727 * t7200 + 2.0_f64 * t27382 * t127218 - 3.0_f64 / 2.0_f64 * t2403 * t8494 * t27777 + t125976 + 3.0_f64 / 2.0_f64 * t2403 * t31859 * t7862 - t1940 * t7091 * t127227 - t1940 * t119737 * t7869 / 2.0_f64 - 3.0_f64 * t126013 * t127233;
    t127236
}
