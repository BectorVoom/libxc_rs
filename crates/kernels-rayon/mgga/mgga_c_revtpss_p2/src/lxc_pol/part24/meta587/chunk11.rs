//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1835/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1835(t1437: f64, t22009: f64, t22321: f64, t4114: f64, t47351: f64, t47395: f64, t5745: f64, t6862: f64, t6874: f64, t75145: f64, t75147: f64, t75176: f64, t75179: f64, t820: f64, t86563: f64, t91942: f64, t92158: f64) -> f64 {
    let t92378 = -0.65854491829355115987e0_f64 * t820 * t1437 * t91942 + 0.87805989105806821314e-1_f64 * t75145 - 0.87805989105806821314e-1_f64 * t75147 - 0.39512695097613069592e1_f64 * t820 * t22321 * t6874 + 0.23707617058567841754e2_f64 * t5745 * t22009 * t6862 + 0.15611904863012452831e0_f64 * t75176 - 0.1561190486301245283e0_f64 * t75179 - t47351 - 0.11708928647259339623e0_f64 * t86563 + 0.39512695097613069591e1_f64 * t820 * t4114 * t92158 - t47395;
    t92378
}
