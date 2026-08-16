//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 914/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk914(t2368: f64, t676: f64, t204: f64, t739: f64, t2509: f64, t724: f64, t2406: f64, t2483: f64, t268: f64) -> (f64, f64, f64, f64, f64) {
    let t9799 = t676 * t2368;
    let t9803 = t204 * t739;
    let t9810 = t676 * t2509;
    let t9814 = t204 * t724;
    let t9820 = 0.53424999999999999999e-1_f64 * t268 * t2483 * t2406;
    (t9799, t9803, t9810, t9814, t9820)
}
