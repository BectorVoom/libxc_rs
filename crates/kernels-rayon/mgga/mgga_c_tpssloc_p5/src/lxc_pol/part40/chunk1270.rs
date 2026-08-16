//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1270/1303 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1270(t4072: f64, t88: f64, t1453: f64, t666: f64, t89: f64, t1441: f64, t1458: f64) -> (f64, f64, f64, f64) {
    let t26117 = t88 * t4072;
    let t26129 = t1453 * t666;
    let t26179 = t89 * t4072;
    let t28002 = t1441 * t1458;
    (t26117, t26129, t26179, t28002)
}
