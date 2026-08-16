//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1240/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1240(t10336: f64, t1920: f64, t1949: f64, t135: f64, t23631: f64, t6688: f64, t23509: f64, t25651: f64, t1016: f64, t3034: f64, t1930: f64, t6741: f64) -> (f64, f64, f64, f64) {
    let t82799 = 0.30461741978670859935e-2_f64 * t1920 * t10336 * t1949;
    let t82822 = t23631 * t135 * t6688;
    let t82895 = t23509 * t25651;
    let t82985 = 1.0_f64 / t3034 / t1016;
    let t82986 = t1930 * t82985;
    let t82987 = t82986 * t6741;
    (t82799, t82822, t82895, t82987)
}
