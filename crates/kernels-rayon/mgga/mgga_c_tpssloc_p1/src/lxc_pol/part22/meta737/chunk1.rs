//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2421/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2421(t17366: f64, t4488: f64, t959: f64, t21091: f64, t2940: f64, t21373: f64, t17930: f64, t4483: f64, t17564: f64, t48890: f64, t1068: f64, t21376: f64, t43637: f64, t4700: f64, t69003: f64, t69005: f64, t69011: f64, t69014: f64, t69018: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t69021 = 0.35089341735807877242e1_f64 * t959 * t4488 * t17366;
    let t69023 = 0.35089341735807877242e1_f64 * t2940 * t21091;
    let t69025 = 0.35089341735807877242e1_f64 * t2940 * t21373;
    let t69027 = 0.10389515463408878255e3_f64 * t4483 * t17930;
    let t69030 = 0.30762056574649219974e4_f64 * t959 * t17564 * t48890;
    let t69031 = -6.0_f64 * t1068 * t21376 * t43637 * t4700 - t69003 + t69005 - t69011 - t69014 + t69018 + t69021 - t69023 + t69025 - t69027 - t69030;
    (t69021, t69023, t69025, t69027, t69030, t69031)
}
