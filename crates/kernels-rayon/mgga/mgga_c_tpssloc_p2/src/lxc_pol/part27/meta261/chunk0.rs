//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1259/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1259(t1437: f64, t79: f64, t72: f64, t1410: f64, t605: f64, t1409: f64, t6500: f64, t6503: f64, t67: f64, t1864: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7431 = t79 * t1437;
    let t7432 = t72 * t7431;
    let t7435 = t605 * t1410;
    let t7440 = 5.0_f64 / 6.0_f64 * t6500 * t1409 + t6503;
    let t7441 = t7440 * t67;
    let t7442 = t7441 * t1864;
    (t7431, t7432, t7435, t7440, t7441, t7442)
}
