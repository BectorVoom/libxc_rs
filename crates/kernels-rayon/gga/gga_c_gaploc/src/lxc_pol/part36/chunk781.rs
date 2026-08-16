//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 781/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk781(t1415: f64, t7030: f64, t9297: f64, t2372: f64, t39776: f64, t900: f64, t2464: f64, t2465: f64, t2487: f64, t9171: f64, t20535: f64, t29969: f64, t4782: f64, t883: f64) -> (f64, f64, f64, f64) {
    let t40106 = t1415 * t9297 * t7030;
    let t40109 = t2372 * t900 * t39776;
    let t40116 = t2487 * t2464 * t2465 * t9171;
    let t40147 = t20535 * t4782 * t883 * t29969;
    (t40106, t40109, t40116, t40147)
}
