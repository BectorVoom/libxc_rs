//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1252/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1252(t34028: f64, t4254: f64, t1518: f64, t32575: f64, t651: f64, t28043: f64, t7359: f64, t34243: f64, t7235: f64, t34251: f64, t7003: f64, t125563: f64, t28196: f64, t28286: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t128557 = 2.0_f64 * t4254 * t34028;
    let t128560 = 2.0_f64 * t651 * t32575 * t1518;
    let t128562 = 2.0_f64 * t7359 * t28043;
    let t128572 = t7235 * t34243;
    let t128574 = 2.0_f64 * t34251 * t7003;
    let t128577 = 2.0_f64 * t28196 * t28286 * t125563;
    (t128557, t128560, t128562, t128572, t128574, t128577)
}
