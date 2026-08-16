//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2054/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2054(t1913: f64, t8130: f64, t2110: f64, t6951: f64, t30626: f64, t575: f64, t1921: f64, t8113: f64, t30663: f64, t571: f64, t104071: f64, t104073: f64, t104077: f64, t104079: f64, t104081: f64, t104083: f64, t104085: f64, t7542: f64) -> f64 {
    let t111408 = t1913 * t8130;
    let t111410 = t2110 * t6951;
    let t111411 = t30626 * t575;
    let t111412 = t8113 * t1921;
    let t111415 = t571 * t30663;
    let t111416 = t6951 * t7542 + t104071 + t104073 + t104077 + t104079 + t104081 + t104083 + t104085 + 2.0_f64 * t111408 + t111410 + t111411 + 2.0_f64 * t111412 + t111415;
    t111416
}
