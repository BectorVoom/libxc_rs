//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1914/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1914(t1468: f64, t1940: f64, t1963: f64, t2403: f64, t25206: f64, t25440: f64, t27158: f64, t27160: f64, t27166: f64, t27169: f64, t27173: f64, t27364: f64, t27368: f64, t27376: f64, t27382: f64, t27385: f64, t27387: f64, t27391: f64, t27395: f64, t27402: f64, t27407: f64, t30: f64, t605: f64, t7010: f64, t7087: f64, t7091: f64, t7092: f64, t7749: f64, t7783: f64, t7787: f64) -> f64 {
    let t27408 = 3.0_f64 * t27158 * t27160 + 3.0_f64 / 2.0_f64 * t2403 * t7087 * t7749 - 3.0_f64 / 2.0_f64 * t25206 * t27166 + 3.0_f64 / 2.0_f64 * t2403 * t1963 * t27169 + 3.0_f64 / 2.0_f64 * t2403 * t1963 * t27173 + 3.0_f64 / 2.0_f64 * t2403 * t7783 * t7010 + t1940 * t27364 * t30 / 2.0_f64 - t1940 * t27368 * t7092 / 2.0_f64 + t1940 * t7783 * t605 / 2.0_f64 - 3.0_f64 / 2.0_f64 * t25206 * t27376 - t1940 * t25440 * t7787 / 2.0_f64 + t27382 * t27385 - t1940 * t7091 * t27387 / 2.0_f64 - t1940 * t7091 * t27391 / 2.0_f64 + 3.0_f64 / 2.0_f64 * t2403 * t1963 * t27395 + t1940 * t7087 * t1468 / 2.0_f64 - t1940 * t7091 * t27402 / 2.0_f64 + t27407;
    t27408
}
