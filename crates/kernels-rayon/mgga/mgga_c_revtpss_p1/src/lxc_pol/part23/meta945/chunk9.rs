//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3113/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3113(t56183: f64, t56236: f64, t58404: f64, t68389: f64, t68399: f64, t81224: f64, t81228: f64, t81230: f64, t81232: f64, t81234: f64, t81236: f64, t43888: f64, t58153: f64, t58165: f64, t58411: f64, t81242: f64, t81245: f64, t81489: f64, t81491: f64, t81494: f64, t81496: f64, t81499: f64, t81501: f64) -> (f64, f64) {
    let t81957 = 0.80513333333333333336e0_f64 * t56183 + 0.543465e1_f64 * t81224 + 0.301925e0_f64 * t81228 - 0.11182407407407407407e0_f64 * t81230 + 0.40256666666666666667e0_f64 * t81232 - 0.60385e0_f64 * t81234 - 0.10064166666666666667e0_f64 * t81236 + t58404 - 0.93932222222222222225e0_f64 * t56236 - 0.30192500000000000001e0_f64 * t68389 + 0.80513333333333333334e0_f64 * t68399;
    let t81969 = 0.10064166666666666667e1_f64 * t81242 - 0.36231e1_f64 * t81245 - 0.49671e0_f64 * t81489 - 0.33114e0_f64 * t81491 + 0.149013e1_f64 * t81494 - 0.24528888888888888889e-1_f64 * t81496 + 0.58258125e1_f64 * t81499 - 0.1237865625e0_f64 * t81501 + t58411 - 0.73586666666666666667e0_f64 * t58153 - 0.91983333333333333333e-1_f64 * t58165 - 0.31310740740740740741e0_f64 * t43888;
    (t81957, t81969)
}
