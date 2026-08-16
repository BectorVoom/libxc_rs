//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1199/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1199(t1518: f64, t572: f64, t670: f64, t8460: f64, t32374: f64, t4292: f64, t5795: f64, t8614: f64, t102005: f64, t28196: f64, t34297: f64, t26399: f64, t7742: f64) -> (f64, f64, f64, f64, f64) {
    let t127459 = 6.0_f64 * t572 * t670 * t8460 * t1518;
    let t127462 = 6.0_f64 * t572 * t32374 * t4292;
    let t127495 = 3.0_f64 * t5795 * t8614;
    let t127532 = 2.0_f64 * t28196 * t102005 * t34297;
    let t127545 = 2.0_f64 * t26399 * t7742;
    (t127459, t127462, t127495, t127532, t127545)
}
