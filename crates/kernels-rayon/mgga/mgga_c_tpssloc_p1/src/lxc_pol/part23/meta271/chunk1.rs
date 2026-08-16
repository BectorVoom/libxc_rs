//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 951/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk951(t20519: f64, t20521: f64, t20525: f64, t20533: f64, t225: f64, t12155: f64, t20356: f64, t5279: f64, t6347: f64, t1347: f64, t20416: f64, t1819: f64, t1821: f64, t5278: f64, t546: f64, t548: f64, t6404: f64, t6408: f64, t6411: f64) -> (f64, f64, f64, f64, f64) {
    let t20536 = (t20519 + t20521 + t20525 + t20533) * t225;
    let t20544 = t12155 * t20356;
    let t20547 = t5279 * t6347;
    let t20550 = t1347 * t20416;
    let t20553 = -36.0_f64 * t1819 * t6408 + 9.0_f64 * t1819 * t6411 + 9.0_f64 * t1821 * t6404 - t20536 * t548 + 60.0_f64 * t20544 * t546 - 36.0_f64 * t20547 * t5278 + 3.0_f64 * t20550 * t546;
    (t20536, t20544, t20547, t20550, t20553)
}
