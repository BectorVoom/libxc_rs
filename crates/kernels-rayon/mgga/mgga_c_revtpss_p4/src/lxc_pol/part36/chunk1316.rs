//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1316/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1316(t23148: f64, t30: f64, t1583: f64, t5962: f64, t25207: f64, t113097: f64, t113100: f64, t113104: f64, t113108: f64, t113111: f64, t113115: f64, t113123: f64, t113416: f64, t113420: f64, t113424: f64, t1940: f64, t1963: f64, t1964: f64, t2403: f64, t25206: f64, t27158: f64, t27368: f64, t27382: f64, t29599: f64, t29602: f64, t29716: f64, t7091: f64, t7783: f64, t98637: f64) -> (f64, f64) {
    let t113428 = t30 * t23148;
    let t113432 = t5962 * t1583;
    let t113433 = t25207 * t113432;
    let t113439 = -9.0_f64 * t27158 * t113097 + 9.0_f64 * t27158 * t113100 - 9.0_f64 / 2.0_f64 * t25206 * t113104 + 3.0_f64 * t27382 * t113108 - 3.0_f64 / 2.0_f64 * t1940 * t7091 * t113111 - 9.0_f64 * t25206 * t113115 - 9.0_f64 * t98637 * t29599 - 3.0_f64 * t1940 * t27368 * t29716 + 3.0_f64 * t113123 * t1964 + t1940 * t113416 * t30 / 2.0_f64 + 9.0_f64 / 2.0_f64 * t2403 * t1963 * t113420 - t1940 * t7091 * t113424 / 2.0_f64 + 3.0_f64 / 2.0_f64 * t2403 * t1963 * t113428 - 9.0_f64 / 2.0_f64 * t25206 * t113433 + 9.0_f64 * t2403 * t7783 * t29602;
    (t113432, t113439)
}
