//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 658/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk658(t438: f64, t4753: f64, t1449: f64, t430: f64, t63: f64, t1452: f64, t71: f64, t4728: f64, t1450: f64, t377: f64, t124: f64, t431: f64) -> (f64, f64, f64, f64, f64) {
    let t4754 = t4753 * t438;
    let t4758 = 1.0_f64 / t1449 / t430;
    let t4759 = t63 * t4758;
    let t4761 = 1.0_f64 / t1452 / t71;
    let t4762 = t4728 * t4761;
    let t4768 = t377 * t1450;
    let t4772 = t124 * t431;
    (t4754, t4759, t4762, t4768, t4772)
}
