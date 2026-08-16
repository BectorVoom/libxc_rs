//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2557/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2557(t21938: f64, t3403: f64, t1117: f64, t21813: f64, t43969: f64, t21810: f64, t3264: f64, t21809: f64, t3315: f64, t3313: f64, t11275: f64, t18265: f64, t4781: f64) -> (f64, f64, f64, f64, f64) {
    let t71672 = t21938 * t3403;
    let t71697 = 0.62071215503128080361e4_f64 * t43969 * t21813 * t1117;
    let t71700 = 2.0_f64 * t3264 * t21810 * t1117;
    let t71701 = t21809 * t3315;
    let t71704 = 0.16081979498692535067e2_f64 * t3313 * t71701 * t1117;
    let t71707 = 0.1551780387578202009e4_f64 * t11275 * t18265 * t4781;
    (t71672, t71697, t71700, t71704, t71707)
}
