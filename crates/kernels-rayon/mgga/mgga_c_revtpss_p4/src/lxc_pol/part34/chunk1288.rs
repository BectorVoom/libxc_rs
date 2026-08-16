//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1288/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1288(t1544: f64, t6079: f64, t27383: f64, t23429: f64, t30: f64, t1468: f64, t5966: f64, t5824: f64, t1583: f64, t106516: f64, t1940: f64, t1963: f64, t22670: f64, t2403: f64, t25206: f64, t25445: f64, t27368: f64, t29591: f64, t29606: f64, t29705: f64, t29713: f64, t29719: f64, t4541: f64, t7091: f64, t7749: f64, t7783: f64, t7787: f64, t92742: f64, t98722: f64) -> (f64, f64) {
    let t113440 = t1544 * t6079;
    let t113441 = t27383 * t113440;
    let t113444 = t30 * t23429;
    let t113454 = t1468 * t5966;
    let t113461 = t5824 * t1544;
    let t113465 = t1468 * t6079;
    let t113484 = t5824 * t1583;
    let t113491 = 9.0_f64 * t25206 * t113441 - 3.0_f64 * t1940 * t92742 * t113444 + 9.0_f64 / 2.0_f64 * t2403 * t7783 * t29606 + 3.0_f64 / 2.0_f64 * t1940 * t7783 * t5824 + 9.0_f64 * t4541 * t1963 * t113454 + 9.0_f64 / 2.0_f64 * t2403 * t29705 * t7749 + 9.0_f64 / 2.0_f64 * t2403 * t1963 * t113461 + 3.0_f64 * t1940 * t25445 * t113465 + 3.0_f64 / 2.0_f64 * t1940 * t29705 * t1468 + 9.0_f64 * t4541 * t7783 * t29591 + 3.0_f64 * t1940 * t98722 * t29713 - 3.0_f64 / 2.0_f64 * t1940 * t27368 * t29719 + t1940 * t1963 * t22670 / 2.0_f64 - 3.0_f64 / 2.0_f64 * t1940 * t7091 * t113484 - 3.0_f64 / 2.0_f64 * t1940 * t106516 * t7787;
    (t113440, t113491)
}
