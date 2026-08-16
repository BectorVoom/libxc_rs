//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1318/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1318(t113439: f64, t113491: f64, t106516: f64, t113096: f64, t113103: f64, t113107: f64, t113415: f64, t113432: f64, t113440: f64, t1544: f64, t1583: f64, t1940: f64, t1962: f64, t1963: f64, t198: f64, t207: f64, t23114: f64, t23148: f64, t23279: f64, t23421: f64, t23429: f64, t2403: f64, t25445: f64, t27368: f64, t29598: f64, t29705: f64, t4541: f64, t5962: f64, t5966: f64, t6075: f64, t6079: f64, t7091: f64, t7783: f64, t892: f64, t92742: f64, t98722: f64) -> (f64, f64) {
    let t113492 = t113439 + t113491;
    let t114089 = -3.0_f64 * t1940 * t106516 * t1583 + 18.0_f64 * t2403 * t25445 * t113440 + 3.0_f64 * t2403 * t1963 * t23148 - 3.0_f64 * t1940 * t27368 * t6075 - t1940 * t7091 * t23421 - 18.0_f64 * t2403 * t27368 * t29598 + 9.0_f64 * t2403 * t29705 * t1544 + 6.0_f64 * t198 * t23114 * t1962 * t892 + 18.0_f64 * t4541 * t7783 * t5966 - 9.0_f64 * t2403 * t7091 * t113432 - 9.0_f64 * t2403 * t7091 * t113103 + 6.0_f64 * t1940 * t25445 * t113107 + 6.0_f64 * t1940 * t98722 * t6079 + 9.0_f64 * t2403 * t7783 * t5962 - 6.0_f64 * t1940 * t92742 * t23429 + t198 * t207 * t113415 * t892 - 18.0_f64 * t4541 * t7091 * t113096 + 18.0_f64 * t4541 * t1963 * t23279;
    (t113492, t114089)
}
