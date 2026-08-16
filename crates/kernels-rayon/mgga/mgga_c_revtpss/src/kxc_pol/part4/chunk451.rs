//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 451/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk451(t5: f64, t1466: f64, t1497: f64, t603: f64, t91: f64, t117: f64, t1468: f64, t100: f64, t55: f64, t108: f64, t105: f64, t109: f64, t97: f64, tau1: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t1501 = piecewise3(t8, 0.0_f64, t1466 * t91 - 4.0_f64 * t1497 * t603);
    let t1502 = t1501 * t117;
    let t1504 = t1468 / 2.0_f64;
    let t1505 = t100 * t1504;
    let t1507 = tau1 * t55;
    let t1509 = -t1504;
    let t1510 = t108 * t1509;
    let t1513 = 5.0_f64 / 3.0_f64 * t105 * t1510 - 5.0_f64 / 3.0_f64 * t1507 * t109 + 5.0_f64 / 3.0_f64 * t97 * t1505;
    (t1501, t1502, t1504, t1505, t1507, t1509, t1513)
}
