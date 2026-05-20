//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3113/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3113<F: Float>(t56183: F, t56236: F, t58404: F, t68389: F, t68399: F, t81224: F, t81228: F, t81230: F, t81232: F, t81234: F, t81236: F, t43888: F, t58153: F, t58165: F, t58411: F, t81242: F, t81245: F, t81489: F, t81491: F, t81494: F, t81496: F, t81499: F, t81501: F) -> (F, F) {
    let t81957 = F::cast_from(0.80513333333333333336e0_f64) * t56183 + F::new(0.543465e1) * t81224 + F::new(0.301925e0) * t81228 - F::cast_from(0.11182407407407407407e0_f64) * t81230 + F::cast_from(0.40256666666666666667e0_f64) * t81232 - F::new(0.60385e0) * t81234 - F::cast_from(0.10064166666666666667e0_f64) * t81236 + t58404 - F::cast_from(0.93932222222222222225e0_f64) * t56236 - F::cast_from(0.30192500000000000001e0_f64) * t68389 + F::cast_from(0.80513333333333333334e0_f64) * t68399;
    let t81969 = F::cast_from(0.10064166666666666667e1_f64) * t81242 - F::new(0.36231e1) * t81245 - F::new(0.49671e0) * t81489 - F::new(0.33114e0) * t81491 + F::new(0.149013e1) * t81494 - F::cast_from(0.24528888888888888889e-1_f64) * t81496 + F::new(0.58258125e1) * t81499 - F::cast_from(0.1237865625e0_f64) * t81501 + t58411 - F::cast_from(0.73586666666666666667e0_f64) * t58153 - F::cast_from(0.91983333333333333333e-1_f64) * t58165 - F::cast_from(0.31310740740740740741e0_f64) * t43888;
    (t81957, t81969)
}
