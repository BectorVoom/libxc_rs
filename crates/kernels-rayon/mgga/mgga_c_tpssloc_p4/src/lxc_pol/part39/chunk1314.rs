//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1314/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1314(t110075: f64, t30053: f64, t29895: f64, t30057: f64, t29900: f64, t30064: f64, t9398: f64, t99: f64, t2196: f64, t2585: f64, t110140: f64, t8181: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t110290 = t110075 * t30053;
    let t110292 = t29895 * t30057;
    let t110294 = t29900 * t30064;
    let t110314 = t99 * t9398;
    let t110333 = 154.0_f64 / 27.0_f64 * t2585 * t2196;
    let t110334 = t110140 * t8181;
    (t110290, t110292, t110294, t110314, t110333, t110334)
}
