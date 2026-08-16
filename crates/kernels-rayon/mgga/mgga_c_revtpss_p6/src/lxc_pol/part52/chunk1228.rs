//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1228/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1228(t28056: f64, t7359: f64, t28696: f64, t8634: f64, t2014: f64, t34242: f64, t7315: f64, t1353: f64, t25082: f64, t28286: f64, t34297: f64, t34270: f64, t7239: f64) -> (f64, f64, f64, f64, f64) {
    let t128204 = 2.0_f64 * t7359 * t28056;
    let t128211 = 2.0_f64 * t8634 * t28696;
    let t128219 = t2014 * t34242 * t7315;
    let t128223 = 6.0_f64 * t25082 * t28286 * t34297 * t1353;
    let t128225 = 3.0_f64 * t34270 * t7239;
    (t128204, t128211, t128219, t128223, t128225)
}
