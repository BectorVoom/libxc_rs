//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 431/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk431(t1024: f64, t1519: f64, t1047: f64, t1503: f64, t1056: f64, t1059: f64, t1510: f64, t1513: f64, t1516: f64, t1062: f64) -> (f64, f64, f64, f64) {
    let t1521 = 1.0_f64 * t1024 * t1519;
    let t1523 = -t1047 + 0.17123333333333333333e-1_f64 * t1503;
    let t1530 = 0.3529725e1_f64 * t1510 - t1056 + 0.516475e0_f64 * t1503 + 0.6311625e0_f64 * t1513 - t1059 + 0.104195e0_f64 * t1516;
    let t1531 = t1530 * t1062;
    (t1521, t1523, t1530, t1531)
}
