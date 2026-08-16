//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1117/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1117(t119706: f64, t119747: f64, t125962: f64, t125968: f64, t125976: f64, t125977: f64, t125980: f64, t125981: f64, t125985: f64, t125988: f64, t125997: f64, t1940: f64, t2403: f64, t25206: f64, t27169: f64, t27173: f64, t27376: f64, t27382: f64, t27385: f64, t27387: f64, t27391: f64, t27395: f64, t27402: f64, t31859: f64, t31863: f64, t31876: f64, t33727: f64, t605: f64, t7092: f64, t7749: f64, t8490: f64, t8494: f64) -> f64 {
    let t126004 = 3.0_f64 / 2.0_f64 * t2403 * t8490 * t27395 - 3.0_f64 / 2.0_f64 * t2403 * t8494 * t27173 + t1940 * t31876 * t27402 - 3.0_f64 / 2.0_f64 * t119747 * t27376 + 2.0_f64 * t27382 * t125962 - t1940 * t31863 * t27387 / 2.0_f64 + t125968 * t27385 + t1940 * t33727 * t605 / 2.0_f64 + t1940 * t31876 * t27391 - t125976 + 3.0_f64 * t119706 * t125977 - 3.0_f64 * t125980 * t125981 - 3.0_f64 * t25206 * t125985 + 3.0_f64 * t119706 * t125988 + 3.0_f64 / 2.0_f64 * t2403 * t31859 * t7749 - 3.0_f64 / 2.0_f64 * t2403 * t8494 * t27169 - t1940 * t125997 * t7092 / 2.0_f64 - t1940 * t31863 * t27391 / 2.0_f64;
    t126004
}
