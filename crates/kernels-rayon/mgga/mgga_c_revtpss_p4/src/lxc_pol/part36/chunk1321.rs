//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1321/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1321(t23421: f64, t33: f64, t113096: f64, t25759: f64, t23148: f64, t1583: f64, t6416: f64, t23429: f64, t1544: f64, t113107: f64, t27799: f64, t113123: f64, t113416: f64, t1940: f64, t1963: f64, t2000: f64, t22783: f64, t2403: f64, t27158: f64, t27368: f64, t27382: f64, t29705: f64, t29939: f64, t29953: f64, t29964: f64, t29970: f64, t4541: f64, t7091: f64, t7783: f64, t7862: f64, t92742: f64, t98722: f64) -> f64 {
    let t114150 = t33 * t23421;
    let t114165 = t25759 * t113096;
    let t114171 = t33 * t23148;
    let t114184 = t6416 * t1583;
    let t114188 = t33 * t23429;
    let t114192 = t6416 * t1544;
    let t114196 = t27799 * t113107;
    let t114199 = -t1940 * t7091 * t114150 / 2.0_f64 + t1940 * t113416 * t33 / 2.0_f64 - 3.0_f64 / 2.0_f64 * t1940 * t27368 * t29970 + 3.0_f64 * t113123 * t2000 + t1940 * t1963 * t22783 / 2.0_f64 - 9.0_f64 * t27158 * t114165 + 9.0_f64 * t4541 * t7783 * t29939 + 3.0_f64 / 2.0_f64 * t2403 * t1963 * t114171 + 3.0_f64 * t1940 * t98722 * t29964 + 9.0_f64 / 2.0_f64 * t2403 * t29705 * t7862 + 9.0_f64 / 2.0_f64 * t2403 * t7783 * t29953 - 3.0_f64 / 2.0_f64 * t1940 * t7091 * t114184 - 3.0_f64 * t1940 * t92742 * t114188 + 9.0_f64 / 2.0_f64 * t2403 * t1963 * t114192 + 3.0_f64 * t27382 * t114196;
    t114199
}
