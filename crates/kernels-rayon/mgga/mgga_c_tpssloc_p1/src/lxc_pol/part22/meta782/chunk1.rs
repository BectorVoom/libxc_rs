//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2673/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2673(t1307: f64, t1345: f64, t1347: f64, t1365: f64, t16186: f64, t16191: f64, t16195: f64, t1819: f64, t19631: f64, t19715: f64, t19728: f64, t19994: f64, t20356: f64, t20416: f64, t20544: f64, t20547: f64, t20550: f64, t5187: f64, t5278: f64, t5279: f64, t546: f64, t6347: f64, t6924: f64, t74355: f64) -> f64 {
    let t74562 = -12.0_f64 * t1307 * t1365 * t20416 * t5278 - 360.0_f64 * t1307 * t20356 * t5278 * t6924 + 3.0_f64 * t1347 * t546 * t74355 + 180.0_f64 * t16191 * t19994 * t5278 - 36.0_f64 * t16195 * t5278 * t6347 - 36.0_f64 * t19631 * t5278 * t5279 + 180.0_f64 * t19715 * t5187 * t5278 + 60.0_f64 * t1345 * t20544 + 3.0_f64 * t1345 * t20550 - 36.0_f64 * t16186 * t20547 + 9.0_f64 * t1819 * t19728;
    t74562
}
