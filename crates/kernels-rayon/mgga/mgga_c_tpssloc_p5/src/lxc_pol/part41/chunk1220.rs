//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1220/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1220(t19440: f64, t72: f64, t1411: f64, t1427: f64, t1434: f64, t19363: f64, t19405: f64, t3968: f64, t3971: f64, t3976: f64, t3998: f64, t4018: f64, t5428: f64, t5442: f64, t609: f64, t629: f64, t642: f64, t66: f64, t80: f64) -> f64 {
    let t19441 = t72 * t19440;
    let t19444 = -t3968 * t1434 / 6.0_f64 - t3971 * t1434 / 6.0_f64 - t1411 * t4018 / 6.0_f64 - t19363 * t80 / 12.0_f64 + t19405 * t80 / 24.0_f64 + t5428 * t642 / 24.0_f64 - t3976 * t1434 / 6.0_f64 + t3998 * t1434 / 12.0_f64 + t1427 * t4018 / 12.0_f64 - t609 * t5442 / 12.0_f64 + t629 * t5442 / 24.0_f64 + t66 * t19441 / 24.0_f64;
    t19444
}
