//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2701/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2701(t4025: f64, t5456: f64, t20193: f64, t604: f64, t1411: f64, t1434: f64, t19322: f64, t19363: f64, t19441: f64, t20207: f64, t20264: f64, t20285: f64, t3962: f64, t3966: f64, t3968: f64, t3971: f64, t3976: f64, t5398: f64, t5442: f64, t55653: f64, t608: f64, t609: f64, t65: f64, t6509: f64, t67: f64, t80: f64) -> (f64, f64, f64) {
    let t75275 = t4025 * t5456;
    let t75284 = t20193 * t604;
    let t75356 = -t608 * t20264 * t80 / 12.0_f64 - t19363 * t1434 / 4.0_f64 - t3976 * t5442 / 4.0_f64 - t609 * t20285 / 12.0_f64 - t3962 * t5442 / 4.0_f64 - t3968 * t5442 / 4.0_f64 - t3971 * t5442 / 4.0_f64 - t1411 * t19441 / 4.0_f64 - t3966 * t65 * t67 * t20207 / 4.0_f64 - t55653 * t20207 / 4.0_f64 - t19322 * t6509 * t5398 / 4.0_f64;
    (t75275, t75284, t75356)
}
