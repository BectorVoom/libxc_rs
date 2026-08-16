//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 788/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk788(t23445: f64, t23486: f64, t23532: f64, t23569: f64, t349: f64, t23346: f64, t23385: f64, t23387: f64, t23389: f64, t23392: f64, t23396: f64, t23399: f64, t23403: f64, t23408: f64, t23410: f64, t388: f64, t6687: f64, t6692: f64) -> (f64, f64) {
    let t23571 = t23445 + t23486 + t23532 + t23569;
    let t23572 = t349 * t23571;
    let t23574 = -0.54831135561607547884e-2_f64 * t23385 - 0.54831135561607547884e-2_f64 * t23387 - 0.14621636149762012769e-1_f64 * t23389 + 0.54831135561607547884e-2_f64 * t23392 + 0.16449340668482264365e-1_f64 * t6687 * t23396 - 0.82246703342411321825e-2_f64 * t6687 * t23399 - 0.54831135561607547884e-2_f64 * t6687 * t23403 - 0.14621636149762012769e-1_f64 * t23346 * t6692 + t23408 * t388 + 2.0_f64 * t23410 * t388 + t23572 * t388;
    (t23571, t23574)
}
