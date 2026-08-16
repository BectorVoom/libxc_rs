//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1492/1505 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1492(t116: f64, t31451: f64, t2212: f64, t5789: f64, t1513: f64, t2: f64, t670: f64, t8406: f64, t1459: f64, t1518: f64, t1916: f64, t21881: f64, t2207: f64, t22559: f64, t22565: f64, t22568: f64, t31234: f64, t31493: f64, t31505: f64, t31506: f64, t31509: f64, t31725: f64, t31731: f64, t31734: f64, t4292: f64, t572: f64, t5802: f64, t5920: f64, t6941: f64, t6945: f64, t8336: f64, t8342: f64, t8346: f64, t8421: f64) -> (f64, f64, f64) {
    let t118137 = t116 * t31451;
    let t118203 = 2.0_f64 * t5789 * t2212;
    let t118374 = t1513 * t2;
    let t118594 = t670 * t8406;
    let t118629 = 12.0_f64 * t118137 * t1518 * t572 + 12.0_f64 * t118594 * t1518 * t572 + 6.0_f64 * t21881 * t572 * t8342 + 6.0_f64 * t31234 * t572 * t5920 + 6.0_f64 * t31493 * t572 * t5920 + 12.0_f64 * t31505 * t4292 * t572 + 6.0_f64 * t1459 * t31725 + 6.0_f64 * t1459 * t31731 + 3.0_f64 * t1459 * t31734 + 12.0_f64 * t1916 * t31506 + 6.0_f64 * t1916 * t31509 + 12.0_f64 * t2207 * t22559 + 6.0_f64 * t2207 * t22565 + 3.0_f64 * t2207 * t22568 + 12.0_f64 * t5802 * t8421 + 3.0_f64 * t6941 * t8346 + 6.0_f64 * t6945 * t8336;
    (t118203, t118374, t118629)
}
