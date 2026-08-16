//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1219/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1219(t19404: f64, t33: f64, t5392: f64, t9321: f64, t2291: f64, t5398: f64, t9330: f64, t2298: f64, t16558: f64, t3966: f64, t4007: f64, t4012: f64, t607: f64, t634: f64, t638: f64) -> (f64, f64) {
    let t19405 = t33 * t19404;
    let t19420 = t9321 * t5392;
    let t19425 = t2291 * t5398;
    let t19430 = t9330 * t5392;
    let t19435 = t2298 * t5398;
    let t19440 = -280.0_f64 / 27.0_f64 * t19420 * t607 + 56.0_f64 / 9.0_f64 * t4007 * t3966 + 28.0_f64 / 9.0_f64 * t19425 * t607 - 4.0_f64 / 3.0_f64 * t634 * t16558 + 280.0_f64 / 27.0_f64 * t19430 * t607 + 56.0_f64 / 9.0_f64 * t4012 * t3966 + 28.0_f64 / 9.0_f64 * t19435 * t607 + 4.0_f64 / 3.0_f64 * t638 * t16558;
    (t19405, t19440)
}
