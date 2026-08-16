//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1084/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1084(t1298: f64, t467: f64, t6576: f64, t814: f64, t11653: f64, t11659: f64, t11602: f64, t11652: f64, t11657: f64, t1268: f64, t1427: f64, t14947: f64, t1674: f64, t1679: f64, t1680: f64, t1734: f64, t2831: f64, t5403: f64, t6589: f64, t6596: f64, t694: f64) -> (f64, f64, f64) {
    let t19409 = t1298 * t467;
    let t19418 = t6576 * t814;
    let t19422 = 0.43374325201206959367e-1_f64 * t11653;
    let t19423 = 0.10843581300301739842e-1_f64 * t11659;
    let t19424 = 2.0_f64 * t1268 * t1679 * t6596 + 24.0_f64 * t1427 * t1674 * t5403 - 2.0_f64 * t1679 * t19418 * t467 - 12.0_f64 * t1680 * t19409 * t694 + 3.0_f64 * t1734 * t2831 * t694 + 24.0_f64 * t14947 * t6589 - t11602 - t11652 + t11657 - t19422 + t19423;
    (t19422, t19423, t19424)
}
