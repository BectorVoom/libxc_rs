//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1270/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1270(t1458: f64, t649: f64, t4072: f64, t88: f64, t1453: f64, t666: f64, t89: f64, t1441: f64, t5493: f64, t5456: f64, t576: f64, t2331: f64, t626: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t26114 = t649 * t1458;
    let t26117 = t88 * t4072;
    let t26129 = t1453 * t666;
    let t26179 = t89 * t4072;
    let t28002 = t1441 * t1458;
    let t28007 = t88 * t5493;
    let t28030 = t89 * t5493;
    let t28893 = t576 * t5456;
    let t29895 = t626 * t2331;
    (t26114, t26117, t26129, t26179, t28002, t28007, t28030, t28893, t29895)
}
